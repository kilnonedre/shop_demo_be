use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait, PaginatorTrait,
};

use crate::{
    entities::coupons::{self, ActiveModel, Model},
    models::{
        coupons::{StructCreateCouponReq, StructUpdateCouponReq, StructUpdateCouponStatusReq},
        StructPagination,
    },
    utils::response::{response_list_t, response_t, ResponseT},
};

/// 创建优惠券
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，优惠券名（必填）
/// - `type` ：整数，类型：0 满减；1 折扣（必填）
/// - `value` ：浮点数，面值（必填）
/// - `total`：整数，发行量（必填）
/// - `used`：整数，使用量（必填）
/// - `min_price`：浮点数，最低使用价格（必填）
/// - `start_time` ：字符串，开始时间（必填）
/// - `end_time` ：字符串，结束时间（必填）
/// - `order` ：整数，权重（必填）
/// - `status` ：整数，用户状态（必填），0 禁用；1 可用
/// - `desc` ：字符串，描述（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新创建的优惠券
/// - 失败：状态码 201，优惠券已存在
/// - 失败：状态码 500
///
/// # 示例
///
/// ```
/// POST /api/admins/manager
/// Connect-Type: application/json
///
/// {
///       "name": "优惠券名称",
///       "type": 0,
///       "value": 20.00,
///       "total": 100,
///       "used": 50,
///       "min_price": 50.00,
///       "start_time": "2022-06-11 20:14:35",
///       "end_time": "2022-10-05 14:01:15",
///       "order": 50,
///       "status": 0,
///       "desc": "描述",
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/coupons",
    request_body = StructCreateCouponReq,
    responses(
        (status = 200, description = "优惠券创建成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "coupons"
)]
pub async fn create_coupon(
    db: web::Data<sea_orm::DatabaseConnection>,
    coupon_data: web::Json<StructCreateCouponReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_coupon = ActiveModel {
        name: Set(coupon_data.name.clone()),
        r#type: Set(coupon_data.r#type),
        value: Set(coupon_data.value),
        total: Set(coupon_data.total),
        used: Set(coupon_data.used),
        min_price: Set(coupon_data.min_price),
        start_time: Set(coupon_data.start_time.clone()),
        end_time: Set(coupon_data.end_time.clone()),
        order: Set(coupon_data.order),
        status: Set(coupon_data.status),
        desc: Set(coupon_data.desc.clone()),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_coupon.insert(db.get_ref()).await;

    match result {
        Ok(new_coupon) => HttpResponse::Ok().json(response_t(Some(200), Some(new_coupon), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改优惠券
///
/// # 路径
///
/// - `id` ：整数，优惠券 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，优惠券名
/// - `type` ：整数，类型，0 满减；1 折扣
/// - `value` ：浮点数，面值
/// - `total`：整数，发行量
/// - `used`：整数，使用量
/// - `min_price`：浮点数，最低使用价格
/// - `start_time` ：字符串，开始时间
/// - `end_time` ：字符串，结束时间
/// - `order` ：整数，权重
/// - `status` ：整数，用户状态，0 禁用；1 可用
/// - `desc` ：字符串，描述
///
/// # 响应
///
/// - 成功：状态码 200，新创建的优惠券
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/coupons/{id}",
    request_body = StructUpdateCouponReq,
    responses(
        (status = 200, description = "优惠券更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "coupons"
)]
pub async fn update_coupon(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    coupon_data: web::Json<StructUpdateCouponReq>,
) -> impl Responder {
    let coupon_result = coupons::Entity::find_by_id(*id).one(db.get_ref()).await;

    match coupon_result {
        Ok(Some(coupon)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_coupon: ActiveModel = coupon.into();
            updated_coupon.name = coupon_data.name.clone().map(Set).unwrap_or(NotSet);
            updated_coupon.r#type = coupon_data.r#type.clone().map(Set).unwrap_or(NotSet);
            updated_coupon.status = coupon_data.status.map(Set).unwrap_or(NotSet);
            updated_coupon.value = coupon_data.value.map(Set).unwrap_or(NotSet);
            updated_coupon.total = coupon_data.total.map(Set).unwrap_or(NotSet);
            updated_coupon.used = coupon_data.used.map(Set).unwrap_or(NotSet);
            updated_coupon.min_price = coupon_data.min_price.map(Set).unwrap_or(NotSet);
            updated_coupon.start_time = coupon_data.start_time.clone().map(Set).unwrap_or(NotSet);
            updated_coupon.end_time = coupon_data.end_time.clone().map(Set).unwrap_or(NotSet);
            updated_coupon.order = coupon_data.order.map(Set).unwrap_or(NotSet);
            updated_coupon.desc = coupon_data.desc.clone().map(Set).unwrap_or(NotSet);
            updated_coupon.update_time = Set(format_time.clone());
            let result = updated_coupon.update(db.get_ref()).await;
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

/// 修改优惠券状态
///
/// # 路径
///
/// - `id` ：整数，优惠券 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `status` ：整数，优惠券状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的角色
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path = "/api/coupons/{id}/update_status",
    request_body = StructUpdateCouponStatusReq,
    responses(
        (status = 200, description = "优惠券更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "coupons"
)]
pub async fn update_coupon_status(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    coupon_data: web::Json<StructUpdateCouponStatusReq>,
) -> impl Responder {
    let coupon_result = coupons::Entity::find_by_id(*id).one(db.get_ref()).await;

    match coupon_result {
        Ok(Some(coupon)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_coupon: ActiveModel = coupon.into();
            updated_coupon.status = Set(coupon_data.status.clone());
            updated_coupon.update_time = Set(format_time.clone());
            let result = updated_coupon.update(db.get_ref()).await;
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

/// 删除优惠券
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
    path  = "/api/coupons/{id}",
    responses(
        (status = 200, description = "角色删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "coupons"
)]
pub async fn delete_coupon(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let result = coupons::Entity::delete_by_id(*id).exec(db.get_ref()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 获取优惠券列表
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/coupons",
    params(
        StructPagination
    ),
    responses(
        (status = 200, description = "角色列表获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "coupons"
)]
pub async fn get_coupon_list(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<StructPagination>,
) -> impl Responder {
    let page = query.page;
    let size = query.size;

    let select = coupons::Entity::find();
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
