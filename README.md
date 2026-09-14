# Rust API Template

Template des APIs Rust de **Mairie 360** (actix-web + utoipa + `mairie360_api_lib`). Il reprend le
squelette commun à `Core_API`, `Project_API`, `Calendar_API`, `Message_API` et `ELearning_API`.

## Créer une nouvelle API

### 1. Renommer l'API et choisir son port

Remplacer `template` / `api_template` / `template-api` / `TEMPLATE`, le port `3000` et l'URL
`template.development.mairie360.fr` partout où figure un marqueur `change api name` ou
`change port` (sur la ligne même, ou juste au-dessus dans les `Dockerfile` qui n'acceptent pas
de commentaire en fin de ligne) :

```bash
grep -rn "change api name\|change port" --exclude-dir=target --exclude-dir=.git .
```

`Cargo.lock` contient aussi le nom du crate : il est régénéré au premier `cargo build`.

Fichiers concernés : `Cargo.toml`, `src/main.rs`, `examples/generate_openapi.rs`,
`tests/queries/**`, `Dockerfile`, `development.Dockerfile`, `entrypoint.sh`, `nginx.conf`,
`docker-compose.yml`, `docker-compose-security.yml`, `docker-compose-performance.yml`,
`.github/workflows/cicd.yml`.

### 2. Activer la CI

Dans `.github/workflows/cicd.yml`, ajouter `push:` sous `on:` et créer les variables
`POSTMAN_<API>_API_COLLECTION_ID` / `POSTMAN_<API>_API_ENV_ID` sur le repo.

### 3. Remplacer la ressource d'exemple

`src/endpoints/v1/example/` (endpoint `GET /api/v1/example/{id}`), `src/database/example/` et
`tests/queries/example/` montrent la structure attendue ; les renommer ou les supprimer. Les
endroits qui les branchent (modules, route `/example`, doc OpenAPI) portent le marqueur
`change example resource`.

### 4. Documenter

Remplir `API.md` et adapter `CLAUDE.md`.

## Commandes

```bash
cargo lint_check      # fmt --check            (CI)
cargo check_code      # clippy -D warnings     (CI)
cargo test            # nécessite Docker + accès ghcr.io/mairie360
cargo cov_test        # llvm-cov, seuil 60 % de lignes (hors endpoints/, main.rs, lib.rs)
cargo open_api > openapi.json && npx orval   # client TypeScript dans generated/
docker compose up --watch                    # stack de dev (Postgres, Liquibase, Redis, nginx)
./security_test.sh    # scan OWASP ZAP
./performance_test.sh # test de charge k6
```

## Renovate

`renovate.json` fusionne automatiquement **toutes** les mises à jour (majeures comprises, 0.x et
Dockerfile de prod inclus) sans attendre ni exiger une CI verte.
