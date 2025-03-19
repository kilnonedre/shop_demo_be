use actix_web::{web, Scope};

use crate::handlers::users::{
    create_user, delete_user, get_user_list, update_user, update_user_status,
};

pub fn build_user_router() -> Scope {
    web::scope("/users")
        .route("", web::get().to(get_user_list))
        .route("", web::post().to(create_user))
        .route("/{id}", web::put().to(update_user))
        .route("/{id}/update_status", web::patch().to(update_user_status))
        .route("/{id}", web::delete().to(delete_user))
}
