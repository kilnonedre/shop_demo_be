use actix_web::{web, Scope};

use crate::handlers::goods::{
    batch_delete_good, batch_restore_good, batch_soft_delete_good, batch_update_good_status,
    create_good, get_good_detail, get_good_list, update_good, update_good_is_check,
};

pub fn build_good_router() -> Scope {
    web::scope("/goods")
        .route("", web::post().to(create_good))
        .route("/{id}", web::get().to(get_good_detail))
        .route("", web::get().to(get_good_list))
        .route("/{id}", web::put().to(update_good))
        .route("/status/batch", web::patch().to(batch_update_good_status))
        .route("/delete/batch", web::patch().to(batch_soft_delete_good))
        .route("/restore/batch", web::patch().to(batch_restore_good))
        .route("/{id}/check", web::patch().to(update_good_is_check))
        .route("", web::delete().to(batch_delete_good))
}
