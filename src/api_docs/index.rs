use serde::Serialize;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{openapi, Modify, OpenApi};

use crate::entities::admins::Model as AdminModel;
use crate::entities::rules::Model as RuleModel;
use crate::models::admins::StructCreateAdmin;

use crate::handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::rules::init_rule,
        handlers::rules::get_rule_list,
        handlers::rules::create_rule,
        handlers::rules::update_rule,
        handlers::rules::update_rule_status,
        handlers::rules::delete_rule,
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
        handlers::skus::get_sku_list,
        handlers::skus::create_sku,
        handlers::skus::update_sku,
        handlers::skus::update_sku_status,
        handlers::skus::delete_all_sku,
        handlers::coupons::get_coupon_list,
        handlers::coupons::create_coupon,
        handlers::coupons::update_coupon,
        handlers::coupons::update_coupon_status,
        handlers::coupons::delete_coupon,
        handlers::user_levels::get_user_level_list,
        handlers::user_levels::create_user_level,
        handlers::user_levels::update_user_level,
        handlers::user_levels::update_user_level_status,
        handlers::user_levels::delete_user_level,
        handlers::images::upload_file,
        handlers::images::delete_all_image,
        handlers::images::update_image,
    ), 
    components(
        schemas(RuleModel, StructCreateAdmin, AdminModel),
    ),
    tags(
        (name = "admins", description = "管理员管理 API"),
        (name = "notices", description = "公告管理 API"),
        (name = "rules", description = "权限管理 API"),
        (name = "roles", description = "角色管理 API"),
        (name = "skus", description = "规格管理 API"),
        (name = "coupons", description = "优惠券管理 API"),
        (name = "users", description = "用户管理 API"),
        (name = "user_levels", description = "会员等级管理 API"),
        (name = "images", description = "图片上传管理 API")
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
