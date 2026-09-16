use utoipa::OpenApi;

// Chaque endpoint ajouté sous `v1/` s'imbrique ici via `nest(...)`, en reprenant le chemin de son
// `config()`. Ne pas mettre de `tags = [...]` dans les `nest` : ils s'accumulent d'un niveau à
// l'autre et l'opération finit avec son tag répété plusieurs fois. Le tag se déclare une seule
// fois, dans le `#[utoipa::path]` de l'endpoint.
//
// Gabarit d'annotation attendu pour un endpoint, à copier dans `<endpoint>/endpoint.rs` :
//
// ```ignore
// #[utoipa::path(
//     post,
//     path = "",
//     summary = "Phrase courte à l'infinitif",
//     description = "Ce que fait l'opération, ce qu'elle ne fait pas, et les pièges : effets de \
//                    bord, champs acceptés mais non persistés, idempotence, valeur par défaut \
//                    d'un champ absent, différence entre un champ absent et un champ `null`.",
//     request_body(
//         content = MaVueDEntree,
//         description = "Ce que porte le corps.",
//         example = json!({ "champ": "valeur" })
//     ),
//     responses(
//         (
//             status = 201,
//             description = "Ce qui a été créé, et ce que le corps contient.",
//             body = MaVueDeSortie,
//             example = json!({ "id": 1 })
//         ),
//         // Une réponse sans `body` documente un corps vide (204, ou 200 sans contenu).
//         (
//             status = 400,
//             description = "Cause précise, pas « Bad request ».",
//             body = String,
//             content_type = "text/plain",
//             example = json!("Message exact du Display de la variante d'erreur")
//         ),
//         (
//             status = 401,
//             description = "En-tête `Authorization` absent, JWT invalide ou expiré, ou session révoquée.",
//             body = String,
//             content_type = "text/plain",
//             example = json!("Jeton expiré")
//         )
//     ),
//     params(
//         ("id" = u64, Path, description = "Ce que l'identifiant désigne.", example = 42)
//     ),
//     security(
//         ("jwt" = [])
//     ),
//     tag = "MonDomaine"
// )]
// ```
//
// Règles qui font la différence entre un contrat complet et un contrat décoratif :
//
// - Ne documenter **que** les statuts réellement atteignables. Un `500` déclaré sur un endpoint
//   qui mappe toutes ses erreurs de base sur `400` est un mensonge que les clients générés
//   propagent. Relire le `status_code()` du `ResponseError` et le corps du handler.
// - Inversement, ne pas oublier `401` sur toute route sous `/api`, et `403` sur les routes qui
//   appliquent un contrôle de droits.
// - Les `example` d'erreur reprennent le message exact du `Display`, pas une paraphrase.
// - `security(("jwt" = []))` suppose que `SecurityAddon` déclare le schéma dans `swagger.rs`.
// - Les champs des `view.rs` portent un `///` et un `#[schema(example = ...)]`, plus les bornes
//   (`min_length`, `maximum`, `pattern`, `format`) : la CI les transforme en schémas zod.
#[derive(OpenApi)]
#[openapi(nest())]
pub struct V1Doc;
