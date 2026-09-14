//! Ressource d'exemple : à renommer ou supprimer en créant l'API.
pub mod doc;
pub mod id;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/example") // change example resource
            .configure(id::config),
    );
}
