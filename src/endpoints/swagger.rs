use crate::endpoints::health::HealthDoc;
use crate::endpoints::hello::HelloDoc;
use crate::endpoints::v1::doc::V1Doc;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

// Le contrat publié ici est la seule source des paquets `@mairie360/<api>-openapi` : tout ce qui
// n'est pas annoté côté Rust est absent des clients TypeScript générés par la CI. À la création
// d'une API, remplacer les marqueurs `#change` ci-dessous et compléter `tags`.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "API Template — Mairie 360", // #change api name
        version = "1.0.0",
        description = "\
Gabarit d'API de la plateforme **Mairie 360**. Remplacer cette description par ce que fait \
réellement le service : son domaine, son vocabulaire, et ce qu'il ne fait pas.

## Authentification

Toutes les routes sous `/api` sont protégées par `JwtMiddleware`. Le jeton s'obtient auprès de \
Core API (`POST /api/v1/auth/login`) et se présente ici dans l'en-tête `Authorization` \
(`Bearer <jwt>`).

Décrire ici les règles d'autorisation propres au service, et **dire explicitement** quand il n'y \
en a pas : un lecteur du contrat ne peut pas le deviner.

## Format des erreurs

Les réponses d'erreur (`4xx` et `5xx`) ont un corps **`text/plain`** contenant le message \
d'erreur, et non un objet JSON — c'est ce que produit `HttpResponse::build(...).body(...)` dans \
les `ResponseError` de chaque endpoint. Toute réponse d'erreur documentée doit donc porter \
`body = String`, `content_type = \"text/plain\"` et un `example` reprenant le message exact du \
`Display` de la variante correspondante.

Statuts renvoyés de façon transverse, avant même d'atteindre le handler :

| Statut | Signification |
| --- | --- |
| `400 Bad Request` | Segment d'URL qui n'est pas un entier, ou corps JSON malformé. |
| `401 Unauthorized` | En-tête `Authorization` absent, malformé, JWT invalide ou expiré, ou session révoquée. |
| `500 Internal Server Error` | Panne de la base de données ou de Redis. |
",
        contact(
            name = "Équipe Mairie 360",
            url = "https://github.com/mairie360"
        ),
        license(
            name = "Propriétaire",
            identifier = "LicenseRef-mairie360-proprietary"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Développement local (cargo run)"), // #change port
        (url = "http://development.mairie360.fr", description = "Pile Docker de développement (nginx)")
    ),
    tags(
        // Un tag par domaine métier, décrit en une phrase. Le `tag` de chaque `#[utoipa::path]`
        // doit correspondre exactement à l'un de ces noms.
        (name = "Service", description = "Sondes techniques non authentifiées, utilisées par Docker et Kubernetes.")
    ),
    nest(
        // Le préfixe doit correspondre au montage réel dans `main.rs` : les routes y sont servies
        // sous `web::scope("/api")`, donc `/api/v1` et non `/v1`.
        (path = "/api/v1", api = V1Doc),
        (path = "/", api = HealthDoc),
        (path = "/", api = HelloDoc),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

/// Déclare le schéma `jwt` référencé par `security(("jwt" = []))` dans les endpoints. Sans lui,
/// ces références pointent dans le vide : Swagger UI n'offre pas de bouton « Authorize » et les
/// clients générés ne savent pas comment s'authentifier.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                Http::builder()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("JWT émis par Core API (`POST /api/v1/auth/login`)."))
                    .build(),
            ),
        )
    }
}
