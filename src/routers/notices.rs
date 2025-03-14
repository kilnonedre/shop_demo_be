use actix_web::{web, Scope};

use crate::handlers::notices::{create_notice, delete_notice, get_notice_list, update_notice};

pub fn build_notice_router() -> Scope {
    web::scope("/notices")
        .route("", web::get().to(get_notice_list))
        .route("", web::post().to(create_notice))
        .route("/{id}", web::put().to(update_notice))
        .route("/{id}", web::delete().to(delete_notice))
}
