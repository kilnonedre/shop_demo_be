use actix_web::{web, App, HttpServer};
use api_docs::index::ApiDoc;
use migration::{Migrator, MigratorTrait};
use routers::rules::build_rule_router;
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
            .service(web::scope("/api").service(build_rule_router()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
