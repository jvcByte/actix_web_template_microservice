use crate::api::home::handler::{check_db_connection, health};
use actix_web::web;

pub fn home_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/health", web::get().to(health))
            .route("/db_conn", web::get().to(check_db_connection)),
    );
}
