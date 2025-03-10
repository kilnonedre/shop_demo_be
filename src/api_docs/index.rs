use serde::Serialize;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{openapi, Modify, OpenApi};

use crate::models::rules::CreateRule;
use crate::entity::rules::Model;

use crate::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::rules::create_rule
    ), 
    components(
        schemas(Model,CreateRule),
    ),
    tags(
        (name = "rules", description = "Rule management API",),
        (name = "rules", description = "Rule management API")
    ),
    modifiers(&Foo),
    security(
        ("api_key1" = ["edit:items", "read:items"], "api_key2" = ["edit:items", "read:items"]),
    )
)]

pub struct ApiDoc;


#[derive(Debug, Serialize)]
struct Foo;

impl Modify for Foo {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "api_key1",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        // .bearer_format("JWT")
                        .build(),
                ),
            );
            schema.add_security_scheme(
                "api_key2",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
