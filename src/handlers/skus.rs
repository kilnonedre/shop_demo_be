use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::{
    entities::skus::{self, ActiveModel, Model},
    models::{
        skus::{CreateSkuReq, DeleteSkuAllReq, UpdateSkuReq, UpdateSkuStatusReq},
        Pagination,
    },
    utils::response::{response_list_t, response_t, ResponseT},
};

/// 创建新规格
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，规格名称（必填）
/// - `status` ：整数，规格状态（必填），0 禁用；1 可用
/// - `order`：整数，规格权重（必填）
/// - `default`：字符串，规格值（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新创建的规格
/// - 失败：状态码 201，规格已存在
/// - 失败：状态码 500
///
/// # 示例
///
/// ```
/// POST /api/admins/manager
/// Connect-Type: application/json
///
/// {
///       "name": "测试",
///       "status": 1,
///       "order": 50,
///       "default": "规格1,规格2",
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/skus",
    request_body = CreateSkuReq,
    responses(
        (status = 200, description = "规格创建成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "skus"
)]
pub async fn create_sku(
    db: web::Data<sea_orm::DatabaseConnection>,
    sku_data: web::Json<CreateSkuReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_sku = ActiveModel {
        name: Set(sku_data.name.clone()),
        status: Set(sku_data.status),
        order: Set(sku_data.order),
        r#type: Set(0),
        default: Set(sku_data.default.clone()),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_sku.insert(db.get_ref()).await;

    match result {
        Ok(new_sku) => HttpResponse::Ok().json(response_t(Some(200), Some(new_sku), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改规格
///
/// # 路径
///
/// - `id` ：整数，规格 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，规格名称（必填）
/// - `status` ：整数，规格状态（必填），0 禁用；1 可用
/// - `order`：整数，规格权重（必填）
/// - `default`：字符串，规格值（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新创建的规格
/// - 失败：状态码 201，规格已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/skus/{id}",
    request_body = UpdateSkuReq,
    responses(
        (status = 200, description = "规格更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "skus"
)]
pub async fn update_sku(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    sku_data: web::Json<UpdateSkuReq>,
) -> impl Responder {
    let sku_result = skus::Entity::find_by_id(*id).one(db.get_ref()).await;

    match sku_result {
        Ok(Some(sku)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_sku: ActiveModel = sku.into();
            updated_sku.name = sku_data.name.clone().map(Set).unwrap_or(NotSet);
            updated_sku.status = sku_data.status.map(Set).unwrap_or(NotSet);
            updated_sku.order = sku_data.order.map(Set).unwrap_or(NotSet);
            updated_sku.default = sku_data.default.clone().map(Set).unwrap_or(NotSet);
            updated_sku.update_time = Set(format_time.clone());
            let result = updated_sku.update(db.get_ref()).await;
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

/// 修改规格状态
///
/// # 路径
///
/// - `id` ：整数，规格 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `status` ：整数，规格状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的规格
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path = "/api/skus/{id}/update_status",
    request_body = UpdateSkuStatusReq,
    responses(
        (status = 200, description = "规格更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "skus"
)]
pub async fn update_sku_status(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    sku_data: web::Json<UpdateSkuStatusReq>,
) -> impl Responder {
    let sku_result = skus::Entity::find_by_id(*id).one(db.get_ref()).await;

    match sku_result {
        Ok(Some(sku)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_sku: ActiveModel = sku.into();
            updated_sku.status = Set(sku_data.status.clone());
            updated_sku.update_time = Set(format_time.clone());
            let result = updated_sku.update(db.get_ref()).await;
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

/// 批量删除规格
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `ids` ：整数，规格状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/skus/delete_all",
    responses(
        (status = 200, description = "规格删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "skus"
)]
pub async fn delete_all_sku(
    db: web::Data<sea_orm::DatabaseConnection>,
    sku_data: web::Json<DeleteSkuAllReq>,
) -> impl Responder {
    let result = skus::Entity::delete_many()
        .filter(skus::Column::Id.is_in(sku_data.ids.clone()))
        .exec(db.get_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 获取规格列表
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/skus",
    params(
        Pagination
    ),
    responses(
        (status = 200, description = "规格列表获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "skus"
)]
pub async fn get_sku_list(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<Pagination>,
) -> impl Responder {
    let page = query.page;
    let size = query.size;

    let select = skus::Entity::find();

    let paginator = select.paginate(db.get_ref(), size);

    let total = match paginator.num_items().await {
        Ok(total) => total,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    };
    let result = paginator.fetch_page(page - 1).await;

    match result {
        Ok(sku_list) => HttpResponse::Ok().json(response_t(
            Some(200),
            Some(response_list_t(sku_list, total)),
            None,
        )),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
