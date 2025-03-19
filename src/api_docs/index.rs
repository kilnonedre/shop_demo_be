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
        handlers::notices::get_notice_list,
        handlers::notices::create_notice,
        handlers::notices::update_notice,
        handlers::notices::delete_notice,
        handlers::roles::get_role_list,
        handlers::roles::create_role,
        handlers::roles::update_role,
        handlers::roles::delete_role,
        handlers::roles::update_role_status,
        handlers::roles::update_role_rule_ids,
        handlers::users::get_user_list,
        handlers::users::create_user,
        handlers::users::update_user,
        handlers::users::update_user_status,
        handlers::users::delete_user,
    ), 
    components(
        schemas(RuleModel, CreateRule, StructCreateAdmin, AdminModel),
    ),
    tags(
        (name = "rules", description = "权限管理 API"),
        (name = "admins", description = "管理员管理 API"),
        (name = "notices", description = "公告管理 API"),
        (name = "roles", description = "角色管理 API"),
        (name = "users", description = "用户管理 API")
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
