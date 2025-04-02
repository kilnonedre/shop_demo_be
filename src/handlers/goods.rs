use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use migration::Expr;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::{
    entities::goods::{self, ActiveModel, Model},
    models::goods::{
        BatchDeleteGoodReq, BatchRestoreGoodReq, BatchSoftDeleteGoodReq, BatchUpdateStatusReq,
        CreateGoodReq, CreateGoodRes, GetGoodListReq, UpdateGoodIsCheckReq, UpdateGoodReq,
    },
    utils::response::{response_list_t, response_t, ResponseT},
};

/// 创建新商品
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `title` ：字符串，名称（必填）
/// - `category_id` ：整数，商品分类 ID（必填）
/// - `cover` ：字符串，封面（必填）
/// - `desc` ：字符串，描述（必填）
/// - `unit` ：字符串，单位（必填）
/// - `stock` ：整数，总库存（必填）
/// - `min_stock` ：整数，库存预警（必填）
/// - `status` ：整数，上架（必填），0 禁用；1 可用
/// - `stock_display` ：整数，库存显示（必填），0 隐藏；1 禁用
/// - `min_price` ：整数，最低售价（必填）
/// - `min_ori_price` ：整数，最低原价（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新创建的商品
/// - 失败：状态码 500
///
/// # 示例
///
/// ```
/// POST /api/admins/manager
/// Connect-Type: application/json
///
/// {
///       "title": "商品名称",
///       "category_id": 0,
///       "cover": "http://...png",
///       "desc": "描述",
///       "unit": "kg",
///       "stock": 200,
///       "min_stock": 10,
///       "status": 1,
///       "stock_display": 1,
///       "min_price": "1.00",
///       "min_ori_price": "100.00",
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/goods",
    request_body = CreateGoodReq,
    responses(
        (status = 200, description = "商品创建成功", body = ResponseT<CreateGoodRes>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "goods"
)]
pub async fn create_good(
    db: web::Data<sea_orm::DatabaseConnection>,
    good_data: web::Json<CreateGoodReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_good = ActiveModel {
        title: Set(good_data.title.clone()),
        category_id: Set(good_data.category_id),
        cover: Set(good_data.cover.clone()),
        desc: Set(good_data.desc.clone()),
        unit: Set(good_data.unit.clone()),
        stock: Set(good_data.stock),
        min_stock: Set(good_data.min_stock),
        status: Set(good_data.status),
        stock_display: Set(good_data.stock_display),
        min_price: Set(good_data.min_price.clone()),
        min_ori_price: Set(good_data.min_ori_price.clone()),
        is_check: Set(0),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_good.insert(db.get_ref()).await;

    match result {
        Ok(new_good) => HttpResponse::Ok().json(response_t(Some(200), Some(new_good), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改商品
///
/// # 路径
///
/// - `id` ：整数，商品 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `title` ：字符串，名称
/// - `category_id` ：整数，商品分类 ID
/// - `cover` ：字符串，封面
/// - `desc` ：字符串，描述
/// - `unit` ：字符串，单位
/// - `stock` ：整数，总库存
/// - `min_stock` ：整数，库存预警
/// - `status` ：整数，上架，0 禁用；1 可用
/// - `stock_display` ：整数，库存显示，0 隐藏；1 禁用
/// - `min_price` ：整数，最低售价
/// - `min_ori_price` ：整数，最低原价
///
/// # 响应
///
/// - 成功：状态码 200，修改之后的商品
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/goods/{id}",
    request_body = UpdateGoodReq,
    responses(
        (status = 200, description = "商品更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误"),
    ),
    tag = "goods"
)]
pub async fn update_good(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    good_data: web::Json<UpdateGoodReq>,
) -> impl Responder {
    let good_result = goods::Entity::find_by_id(*id).one(db.get_ref()).await;
    match good_result {
        Ok(Some(good)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut update_good: ActiveModel = good.into();
            update_good.title = good_data.title.clone().map(Set).unwrap_or(NotSet);
            update_good.category_id = good_data.category_id.map(Set).unwrap_or(NotSet);
            update_good.cover = good_data.cover.clone().map(Set).unwrap_or(NotSet);
            update_good.desc = good_data.desc.clone().map(Set).unwrap_or(NotSet);
            update_good.unit = good_data.unit.clone().map(Set).unwrap_or(NotSet);
            update_good.stock = good_data.stock.map(Set).unwrap_or(NotSet);
            update_good.min_stock = good_data.min_stock.map(Set).unwrap_or(NotSet);
            update_good.status = good_data.status.map(Set).unwrap_or(NotSet);
            update_good.stock_display = good_data.stock_display.map(Set).unwrap_or(NotSet);
            update_good.min_price = good_data.min_price.clone().map(Set).unwrap_or(NotSet);
            update_good.min_ori_price = good_data.min_ori_price.clone().map(Set).unwrap_or(NotSet);
            update_good.update_time = Set(format_time.clone());
            let result = update_good.update(db.get_ref()).await;
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

/// 批量修改商品状态
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `ids` ：整数向量，商品 ID 数组（必填）
/// - `status` ：整数，上架，0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，修改之后的商品
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path = "/api/goods/status/batch",
    request_body = BatchUpdateStatusReq,
    responses(
        (status = 200, description = "商品更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误"),
    ),
    tag = "goods"
)]
pub async fn batch_update_good_status(
    db: web::Data<sea_orm::DatabaseConnection>,
    good_data: web::Json<BatchUpdateStatusReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let good_result = goods::Entity::update_many()
        .col_expr(goods::Column::Status, Expr::value(good_data.status))
        .col_expr(goods::Column::UpdateTime, Expr::value(format_time))
        .filter(goods::Column::Id.is_in(good_data.ids.clone()))
        .exec(db.get_ref())
        .await;
    if let Ok(_) = good_result {
        return HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None));
    }
    return HttpResponse::NotImplemented().json(format!("Error inserting rule"));
}

/// 获取商品列表
///
/// # 查询
///
/// - `page` ：整数，页码（必填）
/// - `size` ：整数，单页数量（必填）
/// - `tab` ：字符串，类型，all 全部；checking 审核中；selling 出售中；off 已下架；min_stock 库存预警；delete 回收站
/// - `title` ：字符串，名称
/// - `category_id` ：整数，商品分类 ID
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path = "/api/goods",
    params(
        GetGoodListReq
    ),
    responses(
        (status = 200, description = "商品列表获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "goods"
)]
pub async fn get_good_list(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<GetGoodListReq>,
) -> impl Responder {
    let mut select = goods::Entity::find();
    if let Some(category_id) = query.category_id {
        select = select.filter(goods::Column::CategoryId.eq(category_id));
    }
    if let Some(title) = query.title.clone() {
        let title_pattern = format!("%{}%", title);
        select = select.filter(goods::Column::Title.like(title_pattern));
    }
    let paginator = select.paginate(db.get_ref(), query.size);

    let total = match paginator.num_items().await {
        Ok(total) => total,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    };
    let result = paginator.fetch_page(query.page - 1).await;

    match result {
        Ok(role_list) => HttpResponse::Ok().json(response_t(
            Some(200),
            Some(response_list_t(role_list, total)),
            None,
        )),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 批量软删除商品
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `ids` ：整数向量，商品 ID 数组（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path  = "/api/goods/delete/batch",
    request_body = BatchSoftDeleteGoodReq,
    responses(
        (status = 200, description = "商品批量删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "goods"
)]
pub async fn batch_soft_delete_good(
    db: web::Data<sea_orm::DatabaseConnection>,
    good_data: web::Json<BatchSoftDeleteGoodReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let good_result = goods::Entity::update_many()
        .col_expr(goods::Column::UpdateTime, Expr::value(&format_time))
        .col_expr(goods::Column::DeleteTime, Expr::value(&format_time))
        .filter(goods::Column::Id.is_in(good_data.ids.clone()))
        .exec(db.get_ref())
        .await;
    if let Ok(_) = good_result {
        return HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None));
    }
    return HttpResponse::NotImplemented().json(format!("Error inserting rule"));
}

/// 批量恢复商品
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `ids` ：整数向量，商品 ID 数组（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path  = "/api/goods/restore/batch",
    request_body = BatchRestoreGoodReq,
    responses(
        (status = 200, description = "商品批量恢复成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "goods"
)]
pub async fn batch_restore_good(
    db: web::Data<sea_orm::DatabaseConnection>,
    good_data: web::Json<BatchRestoreGoodReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let good_result = goods::Entity::update_many()
        .col_expr(goods::Column::UpdateTime, Expr::value(&format_time))
        .col_expr(
            goods::Column::DeleteTime,
            Expr::value(Option::<String>::None),
        )
        .filter(goods::Column::Id.is_in(good_data.ids.clone()))
        .exec(db.get_ref())
        .await;
    if let Ok(_) = good_result {
        return HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None));
    }
    return HttpResponse::NotImplemented().json(format!("Error inserting rule"));
}

/// 批量删除商品
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `ids` ：整数向量，商品 ID 数组（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/goods",
    request_body = BatchDeleteGoodReq,
    responses(
        (status = 200, description = "商品删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "goods"
)]
pub async fn batch_delete_good(
    db: web::Data<sea_orm::DatabaseConnection>,
    good_data: web::Json<BatchDeleteGoodReq>,
) -> impl Responder {
    let result = goods::Entity::delete_many()
        .filter(goods::Column::Id.is_in(good_data.ids.clone()))
        .exec(db.get_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 审核商品
///
/// # 路径
///
/// - `id` ：整数，商品 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `is_check` ：整数，审核（必填），0 未审核；1 同意；2 拒绝
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path  = "/api/goods/{id}/check",
    request_body = UpdateGoodIsCheckReq,
    responses(
        (status = 200, description = "商品审核成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "goods"
)]
pub async fn update_good_is_check(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    good_data: web::Json<UpdateGoodIsCheckReq>,
) -> impl Responder {
    let good_result = goods::Entity::find_by_id(*id).one(db.get_ref()).await;
    match good_result {
        Ok(Some(good)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut update_good: ActiveModel = good.into();
            update_good.is_check = Set(good_data.is_check);
            update_good.update_time = Set(format_time.clone());
            let result = update_good.update(db.get_ref()).await;
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

/// 查看商品资料
///
/// # 路径
///
/// - `id` ：整数，商品 ID（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    get,
    path  = "/api/goods/{id}",
    responses(
        (status = 200, description = "商品审核成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "goods"
)]
pub async fn get_good_detail(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let good_result = goods::Entity::find_by_id(*id).one(db.get_ref()).await;
    match good_result {
        Ok(Some(good_result)) => {
            return HttpResponse::Ok().json(response_t(Some(200), Some(good_result), None));
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
