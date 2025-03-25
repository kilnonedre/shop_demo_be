use actix_web::{web, Scope};

use crate::handlers::rules::{
    create_rule, delete_rule, get_rule_list, init_rule, update_rule, update_rule_status,
};

pub fn build_rule_router() -> Scope {
    web::scope("/rules")
        .route("/init", web::post().to(init_rule))
        .route("", web::get().to(get_rule_list))
        .route("", web::post().to(create_rule))
        .route("/{id}", web::put().to(update_rule))
        .route("/{id}", web::delete().to(delete_rule))
        .route("/{id}/update_status", web::patch().to(update_rule_status))
}
