use actix_web::{web, App, HttpServer};
use api_docs::index::ApiDoc;
use migration::{Migrator, MigratorTrait};
use routers::{
    admins::build_admin_router, coupons::build_coupon_router, goods::build_good_router,
    image_classes::build_image_class_router, images::build_image_router,
    notices::build_notice_router, roles::build_role_router, rules::build_rule_router,
    skus::build_sku_router, user_levels::build_user_level_router, users::build_user_router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api_docs;
mod entities;
mod handlers;
mod models;
mod routers;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let connect =
        sea_orm::Database::connect("postgres://postgres:nmdzz000@localhost:5432/knd_test")
            .await
            .expect("msg");
    Migrator::up(&connect, None).await.expect("msg");

    let db_data = web::Data::new(connect);

    println!("http://127.0.0.1:8080/swagger-ui/");
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(
                web::scope("/api")
                    .service(build_rule_router())
                    .service(build_admin_router())
                    .service(build_notice_router())
                    .service(build_role_router())
                    .service(build_user_router())
                    .service(build_sku_router())
                    .service(build_coupon_router())
                    .service(build_user_level_router())
                    .service(build_image_router())
                    .service(build_image_class_router())
                    .service(build_good_router()),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
