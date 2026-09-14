use api_template::database::example::get::view::{GetExampleQueryResultView, GetExampleQueryView}; // change api name
use mairie360_api_lib::database::db_interface::{ApiRequestDto, Database};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[test]
fn test_query_view_getters() {
    let view = GetExampleQueryView::new(42);

    assert_eq!(view.id(), 42);
    assert_eq!(view.query_params().len(), 1);
    assert!(view.query_sql().contains("FROM users"));
    assert!(format!("{view}").contains("id=42"));
}

#[test]
fn test_result_view_getters_and_display() {
    let result = GetExampleQueryResultView::new(7, "Jane", "Doe", "jane@mairie360.fr");

    assert_eq!(result.id(), 7);
    assert_eq!(result.first_name(), "Jane");
    assert_eq!(result.last_name(), "Doe");
    assert_eq!(result.email(), "jane@mairie360.fr");
    assert!(format!("{result}").contains("Jane"));
}

#[tokio::test]
#[serial]
async fn test_get_example_returns_seeded_user() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let result = db
        .fetch_one::<GetExampleQueryResultView, _>(&GetExampleQueryView::new(1))
        .await
        .expect("query ok");

    assert_eq!(result.id(), 1);
}

#[tokio::test]
#[serial]
async fn test_get_example_unknown_id_is_not_found() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let err = db
        .fetch_one::<GetExampleQueryResultView, _>(&GetExampleQueryView::new(999_999))
        .await
        .unwrap_err();

    assert!(matches!(err, DbError::NotFound));
}
