use crate::endpoints::health::HealthDoc;
use crate::endpoints::v1::doc::V1Doc;
use utoipa::openapi::schema::{ObjectBuilder, Type};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityRequirement, SecurityScheme};
use utoipa::openapi::{ContentBuilder, ResponseBuilder};
use utoipa::{Modify, OpenApi};

/// Name of the JWT security scheme declared in the spec.
pub const BEARER_AUTH: &str = "jwt";

/// Path prefix of the routes wrapped by `JwtMiddleware` in `main.rs`.
const PROTECTED_PREFIX: &str = "/api/";

/// Environment variable that serves Swagger UI (`/swagger-ui/`) and the OpenAPI document
/// (`/api-docs/openapi.json`) when set to `true`. Dev and test stacks set it (ZAP, k6 and newman
/// read the spec from the running API); production leaves it unset, consumers get the contract from
/// the published `@mairie360/<name>-api-openapi` package.
pub const API_DOCS_ENABLED: &str = "API_DOCS_ENABLED";

/// Whether the value of [`API_DOCS_ENABLED`] enables the docs: only `true` (case-insensitive) does,
/// an absent or any other value keeps them off.
#[must_use]
pub fn api_docs_enabled(value: Option<&str>) -> bool {
    value.is_some_and(|v| v.trim().eq_ignore_ascii_case("true"))
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1", api = V1Doc),
        (path = "/", api = HealthDoc),
    ),
    modifiers(&SecurityAddon, &RateLimitAddon)
)]
pub struct ApiDoc;

/// Declares the authentication of every operation, mirroring the `main.rs` wiring: the
/// `jwt` bearer scheme is required at the top level (every `/api/**` route sits behind
/// `JwtMiddleware`), and each operation outside `/api/` is marked public with `security: []`.
///
/// The ZAP OpenAPI coverage gate (mairie360/CICD `tests/zap/zap_hooks.py`) reads this to tell
/// which operations must be reached with a non-401/403 answer, and consumers see which routes
/// need a token.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme(
            BEARER_AUTH,
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some(
                        "Session JWT issued by Core_API: EdDSA (Ed25519) signed with Core's key, \
                             checked against `JWT_PUBLIC_KEY` (header `kid`; claims `sub`, `role`, \
                             `exp`, `iss` `mairie360-core`, `aud` `mairie360`). HS256 tokens are \
                             refused once `JWT_SECRET` is unset (MAIR-680).",
                    ))
                    .build(),
            ),
        );
        openapi.security = Some(vec![SecurityRequirement::new(
            BEARER_AUTH,
            Vec::<String>::new(),
        )]);

        for (path, item) in openapi.paths.paths.iter_mut() {
            if path.starts_with(PROTECTED_PREFIX) {
                continue;
            }
            let operations = [
                &mut item.get,
                &mut item.put,
                &mut item.post,
                &mut item.delete,
                &mut item.options,
                &mut item.head,
                &mut item.patch,
                &mut item.trace,
            ];
            for operation in operations.into_iter().flatten() {
                operation.security = Some(Vec::new());
            }
        }
    }
}

/// Adds the `429` of the per-user rate limiter to every operation that requires a JWT: the limiter
/// answers before any handler runs, so the handlers cannot declare it.
struct RateLimitAddon;

impl Modify for RateLimitAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let too_many_requests = ResponseBuilder::new()
            .description(
                "The caller sent more requests than their rate limit allows (per user, 20 per second \
                 with bursts of 50 by default; the counters are kept per instance of the API). \
                 `Retry-After` and `X-RateLimit-After` give the delay in seconds (at least 1).",
            )
            .content(
                "text/plain",
                ContentBuilder::new()
                    .schema(Some(ObjectBuilder::new().schema_type(Type::String)))
                    .example(Some(serde_json::json!("Too many requests, retry in 1s.")))
                    .build(),
            )
            .build();
        for item in openapi.paths.paths.values_mut() {
            let operations = [
                &mut item.get,
                &mut item.put,
                &mut item.post,
                &mut item.delete,
                &mut item.options,
                &mut item.head,
                &mut item.patch,
                &mut item.trace,
            ];
            for operation in operations.into_iter().flatten() {
                // `None` inherits the global `jwt` requirement; only `security: []` is public.
                if operation.security.as_ref().is_none_or(|s| !s.is_empty()) {
                    operation
                        .responses
                        .responses
                        .insert("429".to_string(), too_many_requests.clone().into());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn spec() -> Value {
        serde_json::from_str(&ApiDoc::openapi().to_json().unwrap()).unwrap()
    }

    #[test]
    fn api_docs_are_off_unless_explicitly_enabled() {
        assert!(api_docs_enabled(Some("true")));
        assert!(api_docs_enabled(Some(" TRUE ")));
        assert!(!api_docs_enabled(None));
        assert!(!api_docs_enabled(Some("")));
        assert!(!api_docs_enabled(Some("1")));
        assert!(!api_docs_enabled(Some("false")));
    }

    #[test]
    fn declares_the_bearer_scheme_at_the_top_level() {
        let spec = spec();
        assert_eq!(
            spec["components"]["securitySchemes"][BEARER_AUTH]["scheme"],
            "bearer"
        );
        assert_eq!(spec["security"], serde_json::json!([{ BEARER_AUTH: [] }]));
    }

    #[test]
    fn marks_every_operation_outside_api_as_public() {
        let spec = spec();
        for (path, item) in spec["paths"].as_object().unwrap() {
            if path.starts_with(PROTECTED_PREFIX) {
                continue;
            }
            for (method, operation) in item.as_object().unwrap() {
                assert_eq!(
                    operation["security"],
                    serde_json::json!([]),
                    "{method} {path} should be public"
                );
            }
        }
    }
}
