//! OpenTelemetry tracing (MAIR-503, `mairie360_api_lib::logging::telemetry`): each request of the
//! API yields a root span carrying its route and status, continues an incoming `traceparent` and
//! leaves without the query string. The redaction of logs and spans is tested in the lib.
//!
//! The template has no business endpoint, so a probe route stands under `/api`, mounted like
//! `main.rs` (`TracingLogger` around the `JwtMiddleware` scope). A new API points these tests at
//! one of its own routes and adds the SQL check of Project API (`db.statement` span events).

use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, test, web, App, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use mairie360_api_lib::jwt_manager::Claims;
use mairie360_api_lib::logging::telemetry::opentelemetry::{global, Value};
use mairie360_api_lib::logging::telemetry::opentelemetry_sdk::propagation::TraceContextPropagator;
use mairie360_api_lib::logging::telemetry::opentelemetry_sdk::trace::{
    InMemorySpanExporter, SdkTracerProvider, SimpleSpanProcessor, SpanData,
};
use mairie360_api_lib::logging::telemetry::{trace_layer, tracer_provider, TracingLogger};
use mairie360_api_lib::security::JwtMiddleware;
use mairie360_api_lib::state::AppState;
use mairie360_api_lib::test_setup::queries_setup::{get_shared_db, ADMIN_ID};
use serial_test::serial;
use tracing::subscriber::DefaultGuard;
use tracing_subscriber::layer::SubscriberExt;

const SECRET: &str = "telemetry-test-secret-of-at-least-32-bytes";
/// Nothing listens on port 1: the tokens carry no session id, so Redis is never read.
const UNREACHABLE_REDIS: &str = "redis://127.0.0.1:1";
const PROBE: &str = "/api/v1/probe/";
const PROBE_SPAN: &str = "GET /api/v1/probe/";

static ENV: Once = Once::new();

/// Stands for any business endpoint of the API.
#[get("/v1/probe/")]
async fn probe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Provider exporting to memory through the lib's redaction, and the subscriber feeding it,
/// installed for the current thread.
fn in_memory_tracing() -> (InMemorySpanExporter, SdkTracerProvider, DefaultGuard) {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let exporter = InMemorySpanExporter::default();
    let provider = tracer_provider(SimpleSpanProcessor::new(exporter.clone()), "template-api"); // change api name
    let guard = tracing::subscriber::set_default(
        tracing_subscriber::registry().with(trace_layer(&provider)),
    );
    (exporter, provider, guard)
}

/// Sends `request` through `TracingLogger` and the JWT gate, and returns the exported root span.
async fn span_of(request: test::TestRequest) -> SpanData {
    ENV.call_once(|| {
        std::env::set_var("JWT_SECRET", SECRET);
        std::env::set_var("JWT_TIMEOUT", "3600");
    });
    let (_, pg_url) = get_shared_db().await;
    let state = AppState::new(UNREACHABLE_REDIS.to_owned(), pg_url.clone()).await;
    let (exporter, provider, _guard) = in_memory_tracing();
    let app = test::init_service(
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(state))
            .service(web::scope("/api").wrap(JwtMiddleware).service(probe)),
    )
    .await;

    // The gate refuses with an `Err`; either way the root span closes once the response is dropped.
    drop(test::try_call_service(&app, request.to_request()).await);

    provider.force_flush().expect("flush the spans");
    let spans = exporter.get_finished_spans().unwrap();
    spans
        .iter()
        .find(|span| span.name == PROBE_SPAN)
        .cloned()
        .unwrap_or_else(|| panic!("no span named {PROBE_SPAN}: {spans:#?}"))
}

fn attribute<'a>(span: &'a SpanData, key: &str) -> Option<&'a Value> {
    span.attributes
        .iter()
        .find(|kv| kv.key.as_str() == key)
        .map(|kv| &kv.value)
}

fn admin_token() -> String {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after the epoch")
        .as_secs()
        + 3600;
    let admin = ADMIN_ID.get().expect("seeded admin");
    encode(
        &Header::default(),
        &Claims::new(
            &admin.to_string(),
            "user",
            usize::try_from(exp).expect("fits"),
        ),
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .expect("encodable token")
}

#[actix_web::test]
#[serial]
async fn a_request_is_exported_as_a_span_continuing_its_trace() {
    get_shared_db().await;
    let trace_id = "4bf92f3577b34da6a3ce929d0e0e4736";
    let span = span_of(
        test::TestRequest::get()
            .uri(&format!("{PROBE}?search=Dupont"))
            .insert_header(("Authorization", format!("Bearer {}", admin_token())))
            .insert_header(("traceparent", format!("00-{trace_id}-00f067aa0ba902b7-01"))),
    )
    .await;

    assert_eq!(
        span.span_context.trace_id().to_string(),
        trace_id,
        "the incoming traceparent must be continued"
    );
    assert_eq!(attribute(&span, "http.status_code"), Some(&Value::I64(200)));
    assert_eq!(attribute(&span, "http.route"), Some(&Value::from(PROBE)));
    assert_eq!(
        attribute(&span, "http.target"),
        Some(&Value::from(PROBE)),
        "the query string must not be exported"
    );
    assert_eq!(attribute(&span, "http.client_ip"), None);
}

#[actix_web::test]
#[serial]
async fn a_refused_request_still_gets_its_span() {
    // No JWT: `JwtMiddleware` refuses before the handler runs.
    let span = span_of(test::TestRequest::get().uri(PROBE)).await;
    assert_eq!(attribute(&span, "http.status_code"), Some(&Value::I64(401)));
}
