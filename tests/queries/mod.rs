// Extension point of the `integration_test` binary (MAIR-533): one `mod <name>;` per query view of the API, each
// file testing the view against the database (`common::api_db_url()`, see `tests/common/mod.rs`). The scaffold has no
// query view yet, so this binary holds no test until the first one is added; do not rely on it to prove anything.
// A query test starts with `use crate::common::...`: declare `mod common;` in `tests/integration_test.rs` first, next to `mod queries;`.
