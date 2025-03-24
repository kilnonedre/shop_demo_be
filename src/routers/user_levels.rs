use actix_web::{web, Scope};

use crate::handlers::user_levels::{
    create_user_level, delete_user_level, get_user_level_list, update_user_level,
    update_user_level_status,
};

pub fn build_user_level_router() -> Scope {
    web::scope("/user_levels")
        .route("", web::get().to(get_user_level_list))
        .route("", web::post().to(create_user_level))
        .route("/{id}", web::put().to(update_user_level))
        .route(
            "/{id}/update_status",
            web::patch().to(update_user_level_status),
        )
        .route("/{id}", web::delete().to(delete_user_level))
}
