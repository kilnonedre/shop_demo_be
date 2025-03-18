use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, PaginatorTrait};

use crate::{
    entities::notices::{self, ActiveModel, Model},
    models::{notices::StructCreateNotice, StructPagination},
    utils::response::{response_list_t, response_t, ResponseT},
};

/// 创建新公告
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `title` ：字符串，标题（必填）
/// - `content` ：字符串，内容（必填）
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
    path  = "/api/notices",
    request_body = StructCreateNotice,
    responses(
        (status = 200, description = "公告创建成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "notices"
)]
pub async fn create_notice(
    db: web::Data<sea_orm::DatabaseConnection>,
    notice_data: web::Json<StructCreateNotice>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_notice = ActiveModel {
        title: Set(notice_data.title.clone()),
        content: Set(notice_data.content.clone()),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_notice.insert(db.get_ref()).await;

    match result {
        Ok(new_notice) => HttpResponse::Ok().json(response_t(Some(200), Some(new_notice), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改公告
///
/// # 路径
///
/// - `id` ：整数，公告 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `title` ：字符串，标题（必填）
/// - `content` ：字符串，内容（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新创建的管理员
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/notices/{id}",
    request_body = StructCreateNotice,
    responses(
        (status = 200, description = "公告更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "notices"
)]
pub async fn update_notice(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    notice_data: web::Json<StructCreateNotice>,
) -> impl Responder {
    let notice_result = notices::Entity::find_by_id(*id).one(db.get_ref()).await;

    match notice_result {
        Ok(Some(notice)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_notice: ActiveModel = notice.into();
            updated_notice.title = Set(notice_data.title.clone());
            updated_notice.content = Set(notice_data.content.clone());
            updated_notice.update_time = Set(format_time.clone());
            let result = updated_notice.update(db.get_ref()).await;
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

/// 删除公告
///
/// # 路径
///
/// - `id` ：整数，公告 ID（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/notices/{id}",
    responses(
        (status = 200, description = "公告删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "notices"
)]
pub async fn delete_notice(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let result = notices::Entity::delete_by_id(*id).exec(db.get_ref()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 获取公告列表
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/notices",
    params(
        ("page", Query, description = "页码，默认值为 1"),
        ("size", Query, description = "每页条目数，默认值为 10")
    ),
    responses(
        (status = 200, description = "公告获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "notices"
)]
pub async fn get_notice_list(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<StructPagination>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let size = query.size.unwrap_or(10);

    let paginator = notices::Entity::find().paginate(db.get_ref(), size);

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
