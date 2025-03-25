use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait, PaginatorTrait,
};

use crate::{
    entities::roles::{self, ActiveModel, Model},
    models::{
        roles::{
            StructCreateRoleReq, StructUpdateRoleReq, StructUpdateRoleRuleIdsReq,
            StructUpdateRoleStatusReq,
        },
        StructPagination,
    },
    utils::response::{response_list_t, response_t, ResponseT},
};

/// 创建新角色
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，角色名（必填）
/// - `desc` ：字符串，角色描述（必填）
/// - `status` ：整数，角色状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的角色
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
///       "name": "角色名称",
///       "desc": "xxxxxxxxxx",
///       "status": 1,
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/roles",
    request_body = StructCreateRoleReq,
    responses(
        (status = 200, description = "角色创建成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "roles"
)]
pub async fn create_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    role_data: web::Json<StructCreateRoleReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_role = ActiveModel {
        name: Set(role_data.name.clone()),
        desc: Set(role_data.desc.clone()),
        status: Set(role_data.status),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_role.insert(db.get_ref()).await;

    match result {
        Ok(new_role) => HttpResponse::Ok().json(response_t(Some(200), Some(new_role), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改角色
///
/// # 路径
///
/// - `id` ：整数，角色 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，角色名（必填）
/// - `desc` ：字符串，角色描述（必填）
/// - `status` ：整数，角色状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的角色
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/roles/{id}",
    request_body = StructUpdateRoleReq,
    responses(
        (status = 200, description = "角色更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "roles"
)]
pub async fn update_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    role_data: web::Json<StructUpdateRoleReq>,
) -> impl Responder {
    let role_result = roles::Entity::find_by_id(*id).one(db.get_ref()).await;

    match role_result {
        Ok(Some(role)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_role: ActiveModel = role.into();
            updated_role.name = role_data.name.clone().map(Set).unwrap_or(NotSet);
            updated_role.desc = role_data.desc.clone().map(Set).unwrap_or(NotSet);
            updated_role.status = role_data.status.map(Set).unwrap_or(NotSet);
            updated_role.update_time = Set(format_time.clone());
            let result = updated_role.update(db.get_ref()).await;
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

/// 删除角色
///
/// # 路径
///
/// - `id` ：整数，角色 ID（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/roles/{id}",
    responses(
        (status = 200, description = "角色删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "roles"
)]
pub async fn delete_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let result = roles::Entity::delete_by_id(*id).exec(db.get_ref()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 获取角色列表
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/roles",
    params(
        ("page", Query, description = "页码，默认值为 1"),
        ("size", Query, description = "每页条目数，默认值为 10")
    ),
    responses(
        (status = 200, description = "角色列表获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "roles"
)]
pub async fn get_role_list(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<StructPagination>,
) -> impl Responder {
    let page = query.page;
    let size = query.size;

    let paginator = roles::Entity::find().paginate(db.get_ref(), size);

    let total = match paginator.num_items().await {
        Ok(total) => total,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    };
    let result = paginator.fetch_page(page - 1).await;

    match result {
        Ok(role_list) => HttpResponse::Ok().json(response_t(
            Some(200),
            Some(response_list_t(role_list, total)),
            None,
        )),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 修改角色状态
///
/// # 路径
///
/// - `id` ：整数，角色 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `status` ：整数，角色状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的角色
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path = "/api/roles/{id}/update_status",
    request_body = StructUpdateRoleStatusReq,
    responses(
        (status = 200, description = "角色更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "roles"
)]
pub async fn update_role_status(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    role_data: web::Json<StructUpdateRoleStatusReq>,
) -> impl Responder {
    let role_result = roles::Entity::find_by_id(*id).one(db.get_ref()).await;

    match role_result {
        Ok(Some(role)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_role: ActiveModel = role.into();
            updated_role.status = Set(role_data.status.clone());
            updated_role.update_time = Set(format_time.clone());
            let result = updated_role.update(db.get_ref()).await;
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

/// 配置角色权限
///
/// # 路径
///
/// - `id` ：整数，角色 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `rule_ids` ：数组<整数>，角色权限（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新创建的角色
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path = "/api/roles/{id}/set_rules",
    request_body = StructUpdateRoleRuleIdsReq,
    responses(
        (status = 200, description = "角色更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "roles"
)]
pub async fn update_role_rule_ids(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    role_data: web::Json<StructUpdateRoleRuleIdsReq>,
) -> impl Responder {
    let role_result = roles::Entity::find_by_id(*id).one(db.get_ref()).await;

    match role_result {
        Ok(Some(role)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_role: ActiveModel = role.into();
            updated_role.rule_ids = Set(Some(role_data.rule_ids.clone()));
            updated_role.update_time = Set(format_time.clone());
            let result = updated_role.update(db.get_ref()).await;
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
