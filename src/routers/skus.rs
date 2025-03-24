use actix_web::{web, Scope};

use crate::handlers::skus::{
    create_sku, delete_all_sku, get_sku_list, update_sku, update_sku_status,
};

pub fn build_sku_router() -> Scope {
    web::scope("/skus")
        .route("", web::get().to(get_sku_list))
        .route("", web::post().to(create_sku))
        .route("/{id}", web::put().to(update_sku))
        .route("/{id}/update_status", web::patch().to(update_sku_status))
        .route("/delete_all", web::delete().to(delete_all_sku))
}
