use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait, PaginatorTrait,
};

use crate::{
    entities::user_levels::{self, ActiveModel, Model},
    models::{
        coupons::{
            StructCreateCouponReq, StructGetCouponListReq, StructUpdateCouponReq,
            StructUpdateCouponStatusReq,
        },
        user_levels::{
            StructCreateUserLevelReq, StructGetUserLevelListReq, StructUpdateUserLevelReq,
            StructUpdateUserLevelStatusReq,
        },
    },
    utils::response::{response_list_t, response_t, ResponseT},
};

/// 创建会员等级
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，会员等级名称（必填）
/// - `level` ：整数，等级（必填）
/// - `status` ：整数，用户状态（必填），0 禁用；1 可用
/// - `discount` ：整数，折扣率 %（必填）
/// - `max_price`：整数，累计消费金额（必填）
/// - `max_time`：整数，累计消费次数（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新创建的会员等级
/// - 失败：状态码 201，会员等级已存在
/// - 失败：状态码 500
///
/// # 示例
///
/// ```
/// POST /api/admins/manager
/// Connect-Type: application/json
///
/// {
///       "name": "会员等级名称",
///       "status": 0,
///       "level": 100,
///       "discount": 10,
///       "max_price": 1000,
///       "max_time": 500,
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/user_levels",
    request_body = StructCreateUserLevelReq,
    responses(
        (status = 200, description = "会员等级创建成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "user_levels"
)]
pub async fn create_user_level(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_level_data: web::Json<StructCreateUserLevelReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_user_level = ActiveModel {
        name: Set(user_level_data.name.clone()),
        level: Set(user_level_data.level),
        status: Set(user_level_data.status),
        discount: Set(user_level_data.discount),
        max_price: Set(user_level_data.max_price),
        max_time: Set(user_level_data.max_time),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_user_level.insert(db.get_ref()).await;

    match result {
        Ok(new_user_level) => {
            HttpResponse::Ok().json(response_t(Some(200), Some(new_user_level), None))
        }
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改会员等级
///
/// # 路径
///
/// - `id` ：整数，会员等级 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，会员等级名称
/// - `level` ：整数，等级
/// - `status` ：整数，用户状态，0 禁用；1 可用
/// - `discount` ：整数，折扣率 %
/// - `max_price`：整数，累计消费金额
/// - `max_time`：整数，累计消费次数
///
/// # 响应
///
/// - 成功：状态码 200，新创建的会员等级
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/user_levels/{id}",
    request_body = StructUpdateUserLevelReq,
    responses(
        (status = 200, description = "会员等级更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "user_levels"
)]
pub async fn update_user_level(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    user_level_data: web::Json<StructUpdateUserLevelReq>,
) -> impl Responder {
    let user_level_result = user_levels::Entity::find_by_id(*id).one(db.get_ref()).await;

    match user_level_result {
        Ok(Some(user_level)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_user_level: ActiveModel = user_level.into();
            updated_user_level.name = user_level_data.name.clone().map(Set).unwrap_or(NotSet);
            updated_user_level.level = user_level_data.level.map(Set).unwrap_or(NotSet);
            updated_user_level.status = user_level_data.status.map(Set).unwrap_or(NotSet);
            updated_user_level.discount = user_level_data.discount.map(Set).unwrap_or(NotSet);
            updated_user_level.max_price = user_level_data.max_price.map(Set).unwrap_or(NotSet);
            updated_user_level.max_time = user_level_data.max_time.map(Set).unwrap_or(NotSet);
            updated_user_level.update_time = Set(format_time.clone());
            let result = updated_user_level.update(db.get_ref()).await;
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

/// 修改会员等级状态
///
/// # 路径
///
/// - `id` ：整数，会员等级 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `status` ：整数，会员等级状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的会员等级
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path = "/api/user_levels/{id}/update_status",
    request_body = StructUpdateUserLevelStatusReq,
    responses(
        (status = 200, description = "优惠券更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "user_levels"
)]
pub async fn update_user_level_status(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    user_level_data: web::Json<StructUpdateUserLevelStatusReq>,
) -> impl Responder {
    let user_level_result = user_levels::Entity::find_by_id(*id).one(db.get_ref()).await;

    match user_level_result {
        Ok(Some(user_level)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_user_level: ActiveModel = user_level.into();
            updated_user_level.status = Set(user_level_data.status.clone());
            updated_user_level.update_time = Set(format_time.clone());
            let result = updated_user_level.update(db.get_ref()).await;
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

/// 删除会员等级
///
/// # 路径
///
/// - `id` ：整数，会员等级 ID（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/user_levels/{id}",
    responses(
        (status = 200, description = "会员等级删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "user_levels"
)]
pub async fn delete_user_level(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let result = user_levels::Entity::delete_by_id(*id)
        .exec(db.get_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 获取会员等级列表
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/user_levels",
    params(
        StructGetUserLevelListReq
    ),
    responses(
        (status = 200, description = "会员等级列表获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "user_levels"
)]
pub async fn get_user_level_list(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<StructGetUserLevelListReq>,
) -> impl Responder {
    let page = query.page;
    let size = query.size;

    let select = user_levels::Entity::find();
    let paginator = select.paginate(db.get_ref(), size);

    let total = match paginator.num_items().await {
        Ok(total) => total,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    };
    let result = paginator.fetch_page(page - 1).await;

    match result {
        Ok(user_level_list) => HttpResponse::Ok().json(response_t(
            Some(200),
            Some(response_list_t(user_level_list, total)),
            None,
        )),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
