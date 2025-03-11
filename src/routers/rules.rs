use actix_web::{web, Scope};

use crate::handlers::rules::{create_rule, init_rule};

pub fn build_rule_router() -> Scope {
    web::scope("/rules")
        .route("", web::post().to(create_rule))
        .route("/init", web::post().to(init_rule))
}
