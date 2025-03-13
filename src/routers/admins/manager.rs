use actix_web::{web, Scope};

use crate::handlers::admins::{create_admin, delete_admin, update_admin, update_admin_status};

pub fn build_admin_manager_router() -> Scope {
    web::scope("/manager")
        .route("", web::post().to(create_admin))
        .route("/{id}", web::put().to(update_admin))
        .route("/{id}", web::delete().to(delete_admin))
        .route("/{id}/update_status", web::patch().to(update_admin_status))
}
