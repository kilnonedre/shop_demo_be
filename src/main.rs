use actix_web::{web, App, HttpServer};
use api_doc::ApiDoc;
use handlers::rule::create_rule;
use migration::{Migrator, MigratorTrait};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api_doc;
mod entity;
mod handlers;

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
                    .service(web::scope("/rules").route("", web::post().to(create_rule))),
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
