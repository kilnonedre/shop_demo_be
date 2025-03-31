use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::{
    entities::users::{self, ActiveModel, Model},
    models::users::{
        CreateUserReq, GetUserListReq, UpdateUserReq, UpdateUserStatusReq,
    },
    utils::response::{response_list_t, response_t, ResponseT},
};

/// 创建新用户
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `username` ：字符串，用户名（必填）
/// - `password` ：字符串，密码（必填）
/// - `status` ：整数，用户状态（必填），0 禁用；1 可用
/// - `user_level_id`：整数，会员等级 ID（必填）
/// - `nickname`：字符串，昵称
/// - `phone`：字符串，手机号
/// - `email`：字符串，邮箱
/// - `avatar`：字符串，头像
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
///       "username": "测试",
///       "password": "test_test_test",
///       "status": 1,
///       "user_level_id": 1,
///       "nickname": null,
///       "phone": null,
///       "email": null,
///       "avatar": null,
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/users",
    request_body = CreateUserReq,
    responses(
        (status = 200, description = "角色创建成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "users"
)]
pub async fn create_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_data: web::Json<CreateUserReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_user = ActiveModel {
        username: Set(user_data.username.clone()),
        password: Set(user_data.password.clone()),
        status: Set(user_data.status),
        user_level_id: Set(user_data.user_level_id),
        nickname: Set(user_data.nickname.clone()),
        phone: Set(user_data.phone.clone()),
        email: Set(user_data.email.clone()),
        avatar: Set(user_data.avatar.clone()),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_user.insert(db.get_ref()).await;

    match result {
        Ok(new_user) => HttpResponse::Ok().json(response_t(Some(200), Some(new_user), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改用户
///
/// # 路径
///
/// - `id` ：整数，用户 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `username` ：字符串，用户名
/// - `password` ：字符串，密码
/// - `status` ：整数，用户状态，0 禁用；1 可用
/// - `user_level_id`：整数，会员等级 ID
/// - `nickname`：字符串，昵称
/// - `phone`：字符串，手机号
/// - `email`：字符串，邮箱
/// - `avatar`：字符串，头像
///
/// # 响应
///
/// - 成功：状态码 200，新创建的角色
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    request_body = UpdateUserReq,
    responses(
        (status = 200, description = "用户更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "users"
)]
pub async fn update_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    user_data: web::Json<UpdateUserReq>,
) -> impl Responder {
    let user_result = users::Entity::find_by_id(*id).one(db.get_ref()).await;

    match user_result {
        Ok(Some(user)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_user: ActiveModel = user.into();
            updated_user.username = user_data.username.clone().map(Set).unwrap_or(NotSet);
            updated_user.password = user_data.password.clone().map(Set).unwrap_or(NotSet);
            updated_user.status = user_data.status.map(Set).unwrap_or(NotSet);
            updated_user.user_level_id = user_data.user_level_id.map(Set).unwrap_or(NotSet);
            updated_user.nickname = user_data
                .nickname
                .clone()
                .map(|nickname| Set(Some(nickname)))
                .unwrap_or(NotSet);
            updated_user.phone = user_data
                .phone
                .clone()
                .map(|phone| Set(Some(phone)))
                .unwrap_or(NotSet);
            updated_user.email = user_data
                .email
                .clone()
                .map(|email| Set(Some(email)))
                .unwrap_or(NotSet);
            updated_user.avatar = user_data
                .avatar
                .clone()
                .map(|avatar| Set(Some(avatar)))
                .unwrap_or(NotSet);
            updated_user.update_time = Set(format_time.clone());
            let result = updated_user.update(db.get_ref()).await;
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

/// 修改用户状态
///
/// # 路径
///
/// - `id` ：整数，用户 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `status` ：整数，用户状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的角色
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path = "/api/users/{id}/update_status",
    request_body = UpdateUserStatusReq,
    responses(
        (status = 200, description = "角色更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "users"
)]
pub async fn update_user_status(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    user_data: web::Json<UpdateUserStatusReq>,
) -> impl Responder {
    let role_result = users::Entity::find_by_id(*id).one(db.get_ref()).await;

    match role_result {
        Ok(Some(user)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_user: ActiveModel = user.into();
            updated_user.status = Set(user_data.status.clone());
            updated_user.update_time = Set(format_time.clone());
            let result = updated_user.update(db.get_ref()).await;
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

/// 删除用户
///
/// # 路径
///
/// - `id` ：整数，用户 ID（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/users/{id}",
    responses(
        (status = 200, description = "角色删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "users"
)]
pub async fn delete_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let result = users::Entity::delete_by_id(*id).exec(db.get_ref()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 获取用户列表
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/users",
    params(
        GetUserListReq
    ),
    responses(
        (status = 200, description = "角色列表获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "users"
)]
pub async fn get_user_list(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<GetUserListReq>,
) -> impl Responder {
    let page = query.page;
    let size = query.size;

    let mut select = users::Entity::find();
    if let Some(user_level_id) = query.user_level_id {
        select = select.filter(users::Column::UserLevelId.eq(user_level_id));
    }
    if let Some(keyword) = query.keyword.clone() {
        let keyword_pattern = format!("%{}%", keyword);
        select = select.filter(
            users::Column::Username
                .like(&keyword_pattern)
                .or(users::Column::Phone
                    .like(&keyword_pattern)
                    .or(users::Column::Email.like(&keyword_pattern))),
        )
    }
    let paginator = select.paginate(db.get_ref(), size);

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
