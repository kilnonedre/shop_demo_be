use actix_web::{web, Scope};

use crate::handlers::coupons::{
    create_coupon, delete_coupon, get_coupon_list, update_coupon, update_coupon_status,
};

pub fn build_coupon_router() -> Scope {
    web::scope("/coupons")
        .route("", web::get().to(get_coupon_list))
        .route("", web::post().to(create_coupon))
        .route("/{id}", web::put().to(update_coupon))
        .route("/{id}/update_status", web::patch().to(update_coupon_status))
        .route("/{id}", web::delete().to(delete_coupon))
}
