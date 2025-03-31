use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    entities::admins::{self, ActiveModel, Model},
    models::admins::{CreateAdmin, UpdateAdminStatus},
    utils::response::{response_t, ResponseT},
};

/// 创建新管理员
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `username` ：字符串，用户名（必填）
/// - `password` ：字符串，密码（必填）
/// - `role_id` ：整数，角色表 ID（必填）
/// - `status` ：整数，管理员状态（必填），0 禁用；1 可用
/// - `avatar` ：字符串，头像
///
/// # 响应
///
/// - 成功：状态码 200，新创建的管理员
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
/// # 示例
///
/// ```
/// POST /api/admins/manager
/// Connect-Type: application/json
///
/// {
///       "avatar": null,
///       "password": "xxxxxxxxxx",
///       "role_id": 38,
///       "status": 1,
///       "username": "admin2"
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/admins/manager",
    request_body = CreateAdmin,
    responses(
        (status = 200, description = "管理员创建成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "admins"
)]
pub async fn create_admin(
    db: web::Data<sea_orm::DatabaseConnection>,
    admin_data: web::Json<CreateAdmin>,
) -> impl Responder {
    let admin_result = admins::Entity::find()
        .filter(admins::Column::Username.eq(admin_data.username.clone()))
        .one(db.get_ref())
        .await;
    if let Ok(Some(_)) = admin_result {
        return HttpResponse::Ok().json(response_t::<()>(
            Some(201),
            None,
            Some(String::from("username已存在")),
        ));
    }
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_admin = ActiveModel {
        username: Set(admin_data.username.clone()),
        password: Set(admin_data.password.clone()),
        role_id: Set(admin_data.role_id),
        status: Set(admin_data.status),
        avatar: Set(admin_data.avatar.clone()),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        is_super: Set(0),
        ..Default::default()
    };
    let result = new_admin.insert(db.get_ref()).await;

    match result {
        Ok(new_admin) => HttpResponse::Ok().json(response_t(Some(200), Some(new_admin), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改管理员
///
/// # 路径
///
/// - `id` ：整数，管理员ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `username` ：字符串，用户名（必填）
/// - `password` ：字符串，密码（必填）
/// - `role_id` ：整数，角色表 ID（必填）
/// - `status` ：整数，管理员状态（必填），0 禁用；1 可用
/// - `avatar` ：字符串，头像
///
/// # 响应
///
/// - 成功：状态码 200，新创建的管理员
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path  = "/api/admins/manager/{id}",
    request_body = CreateAdmin,
    responses(
        (status = 200, description = "管理员更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "admins"
)]
pub async fn update_admin(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    admin_data: web::Json<CreateAdmin>,
) -> impl Responder {
    let admin_result = admins::Entity::find_by_id(*id).one(db.get_ref()).await;

    match admin_result {
        Ok(Some(admin)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_admin: ActiveModel = admin.into();
            updated_admin.username = Set(admin_data.username.clone());
            updated_admin.role_id = Set(admin_data.role_id);
            updated_admin.status = Set(admin_data.status);
            updated_admin.avatar = Set(admin_data.avatar.clone());
            updated_admin.update_time = Set(format_time.clone());
            let result = updated_admin.update(db.get_ref()).await;
            if let Ok(_) = result {
                return HttpResponse::Ok().json(response_t(
                    Some(200),
                    Some(String::from("OK")),
                    None,
                ));
            }
            return HttpResponse::NotImplemented().json(format!("Error inserting rule"));
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 删除管理员
///
/// # 路径
///
/// - `id` ：整数，管理员ID（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/admins/manager/{id}",
    responses(
        (status = 200, description = "管理员删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "admins"
)]
pub async fn delete_admin(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let result = admins::Entity::delete_by_id(*id).exec(db.get_ref()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 修改管理员状态
///
/// # 路径
///
/// - `id` ：整数，管理员ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `status` ：整数，管理员状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的管理员
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path  = "/api/admins/manager/{id}/update_status",
    request_body = UpdateAdminStatus,
    responses(
        (status = 200, description = "管理员状态修改成功", body = ResponseT<String>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "admins"
)]
pub async fn update_admin_status(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    admin_data: web::Json<UpdateAdminStatus>,
) -> impl Responder {
    let admin_result = admins::Entity::find_by_id(*id).one(db.get_ref()).await;

    match admin_result {
        Ok(Some(admin)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_admin: ActiveModel = admin.into();
            updated_admin.status = Set(admin_data.status);
            updated_admin.update_time = Set(format_time.clone());
            let result = updated_admin.update(db.get_ref()).await;
            if let Ok(_) = result {
                return HttpResponse::Ok().json(response_t(
                    Some(200),
                    Some(String::from("OK")),
                    None,
                ));
            }
            return HttpResponse::NotImplemented().json(format!("Error inserting rule"));
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
