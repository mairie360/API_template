pub mod doc;
pub mod get;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/{id}").service(get::endpoint::get_example));
}
