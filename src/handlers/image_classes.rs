use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::{
    entities::{
        image_classes::{self, ActiveModel, Model},
        images,
    },
    models::{
        image_classes::{StructCreateImageClassReq, StructUpdateImageClassReq},
        StructPagination,
    },
    utils::response::{response_list_t, response_t, ResponseT},
};

/// 创建新图库
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，图库名称（必填）
/// - `order` ：整数，图库权重（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新创建的图库
/// - 失败：状态码 201，图库已存在
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
    path  = "/api/image_classes",
    request_body = StructCreateImageClassReq,
    responses(
        (status = 200, description = "图库创建成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "image_classes"
)]
pub async fn create_image_class(
    db: web::Data<sea_orm::DatabaseConnection>,
    image_class_data: web::Json<StructCreateImageClassReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_image_class = ActiveModel {
        name: Set(image_class_data.name.clone()),
        order: Set(image_class_data.order),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_image_class.insert(db.get_ref()).await;

    match result {
        Ok(new_image_class) => {
            HttpResponse::Ok().json(response_t(Some(200), Some(new_image_class), None))
        }
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改图库
///
/// # 路径
///
/// - `id` ：整数，图库 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，图库名称
/// - `order` ：整数，图库权重
///
/// # 响应
///
/// - 成功：状态码 200，修改之后的图库
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/image_classes/{id}",
    request_body = StructUpdateImageClassReq,
    responses(
        (status = 200, description = "图库更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "image_classes"
)]
pub async fn update_image_class(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    image_class_data: web::Json<StructUpdateImageClassReq>,
) -> impl Responder {
    let image_class_result = image_classes::Entity::find_by_id(*id)
        .one(db.get_ref())
        .await;

    match image_class_result {
        Ok(Some(image_class)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_image_class: ActiveModel = image_class.into();
            updated_image_class.name = image_class_data.name.clone().map(Set).unwrap_or(NotSet);
            updated_image_class.order = image_class_data.order.map(Set).unwrap_or(NotSet);
            updated_image_class.update_time = Set(format_time.clone());
            let result = updated_image_class.update(db.get_ref()).await;
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

/// 删除图库
///
/// # 路径
///
/// - `id` ：整数，图库 ID（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/image_classes/{id}",
    responses(
        (status = 200, description = "图库删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "image_classes"
)]
pub async fn delete_image_class(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let result = image_classes::Entity::delete_by_id(*id)
        .exec(db.get_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 获取图库列表
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/image_classes",
    params(
        StructPagination
    ),
    responses(
        (status = 200, description = "图库列表获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "image_classes"
)]
pub async fn get_image_class_list(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<StructPagination>,
) -> impl Responder {
    let page = query.page;
    let size = query.size;

    let paginator = image_classes::Entity::find().paginate(db.get_ref(), size);

    let total = match paginator.num_items().await {
        Ok(total) => total,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    };
    let result = paginator.fetch_page(page - 1).await;

    match result {
        Ok(image_class_list) => HttpResponse::Ok().json(response_t(
            Some(200),
            Some(response_list_t(image_class_list, total)),
            None,
        )),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 获取图库下的图片列表
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/image_classes/{id}/image",
    params(
        StructPagination
    ),
    responses(
        (status = 200, description = "图库下的图片列表获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "image_classes"
)]
pub async fn get_image_list_by_image_class_id(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    query: web::Query<StructPagination>,
) -> impl Responder {
    let page = query.page;
    let size = query.size;

    let paginator = images::Entity::find()
        .filter(images::Column::ImageClassId.eq(*id))
        .paginate(db.get_ref(), size);

    let total = match paginator.num_items().await {
        Ok(total) => total,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    };
    let result = paginator.fetch_page(page - 1).await;

    match result {
        Ok(image_class_list) => HttpResponse::Ok().json(response_t(
            Some(200),
            Some(response_list_t(image_class_list, total)),
            None,
        )),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
