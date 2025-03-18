use actix_web::{web, Scope};

use crate::handlers::roles::{
    create_role, delete_role, get_role_list, update_role, update_role_rule_ids, update_role_status,
};

pub fn build_role_router() -> Scope {
    web::scope("/roles")
        .route("", web::get().to(get_role_list))
        .route("", web::post().to(create_role))
        .route("/{id}", web::put().to(update_role))
        .route("/{id}", web::delete().to(delete_role))
        .route("/{id}/update_status", web::patch().to(update_role_status))
        .route("/{id}/set_rules", web::patch().to(update_role_rule_ids))
}
