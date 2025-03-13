use serde::Serialize;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{openapi, Modify, OpenApi};

use crate::entities::admins::Model as AdminModel;
use crate::entities::rules::Model as RuleModel;
use crate::models::admins::StructCreateAdmin;
use crate::models::rules::CreateRule;

use crate::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::rules::create_rule,
        handlers::rules::init_rule,
        handlers::admins::create_admin,
        handlers::admins::update_admin,
        handlers::admins::delete_admin,
        handlers::admins::update_admin_status,
    ), 
    components(
        schemas(RuleModel, CreateRule, StructCreateAdmin, AdminModel),
    ),
    tags(
        (name = "rules", description = "Rule management API"),
        (name = "admins", description = "Rule management API")
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
