use actix_web::{web, Scope};

use crate::handlers::images::{delete_all_image, update_image, upload_file};

pub fn build_image_router() -> Scope {
    web::scope("/images")
        .route("/upload", web::post().to(upload_file))
        .route("/delete_all", web::delete().to(delete_all_image))
        .route("/{id}", web::put().to(update_image))
}
