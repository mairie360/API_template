pub mod doc;
pub mod example; // change example resource

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").configure(example::config)); // change example resource
}
