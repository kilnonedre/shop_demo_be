use actix_web::{web, Scope};

use crate::handlers::image_classes::{
    create_image_class, delete_image_class, get_image_class_list, get_image_list_by_image_class_id,
    update_image_class,
};

pub fn build_image_class_router() -> Scope {
    web::scope("/image_classes")
        .route("", web::get().to(get_image_class_list))
        .route(
            "/{id}/image",
            web::get().to(get_image_list_by_image_class_id),
        )
        .route("", web::post().to(create_image_class))
        .route("/{id}", web::put().to(update_image_class))
        .route("/{id}", web::delete().to(delete_image_class))
}
