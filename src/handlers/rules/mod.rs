use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait,
};
use utils::{build_rule_tree, insert_rule_with_child};

use crate::{
    entities::rules::{self, ActiveModel, Model},
    models::rules::{
        CreateRuleReq, Rule, UpdateRuleReq, UpdateRuleStatusReq,
    },
    utils::{
        json::read_json_from_file,
        response::{response_list_t, response_t, ResponseT},
    },
};

mod utils;

/// 初始化规则
///
/// # 请求体
///
/// 无
///
/// # 响应
///
/// - 成功：返回状态码 200 和新创建的规则对象
/// - 失败：返回状态码 500
///
/// # 示例
///
/// ```
/// POST /rules
/// Connect-Type: application/json
///
/// {
///     "name": "测试"
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/rules/init",
    responses(
        (status = 200, description = "Rule created successfully", body = Model),
        (status = 500, description = "Internal server error")
    ),
    tag = "rules"
)]
pub async fn init_rule(db: web::Data<sea_orm::DatabaseConnection>) -> impl Responder {
    let result = read_json_from_file::<Vec<Rule>>("./src/assets/rule.json");
    match result {
        Ok(rules) => {
            insert_rule_with_child(&db, rules).await;
            HttpResponse::Ok().json("规则已初始化")
        }
        Err(err) => {
            println!("发生错误: {}", err);
            HttpResponse::InternalServerError().json("初始化规则失败")
        }
    }
}

/// 创建新规则
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `rule_id` ：整数，上级菜单 ID（必填）
/// - `status` ：整数，状态（必填），0 禁用；1 可用
/// - `name` ：字符串，菜单/权限名称（必填）
/// - `front_path` ：字符串，前端路由路径（必填）
/// - `condition` ：字符串，后端规则，由后端提供的接口别名（必填）
/// - `menu` ：整数，是否为菜单（必填），0 否；1 是
/// - `order` ：整数，权重（必填）
/// - `icon` ：字符串，element-plus 图标（必填）
/// - `method` ：字符串，请求方式（必填）
///
/// # 响应
///
/// - 成功：返回状态码 200 和新创建的规则对象
/// - 失败：返回状态码 500
///
/// # 示例
///
/// ```
/// POST /rules
/// Connect-Type: application/json
///
/// {
///     "name": "测试"
/// }
/// ```
#[utoipa::path(
    post,
    path  = "/api/rules",
    request_body = CreateRuleReq,
    responses(
        (status = 200, description = "Rule created successfully", body = ResponseT<Model>),
        (status = 500, description = "Internal server error")
    ),
    tag = "rules"
)]
pub async fn create_rule(
    db: web::Data<sea_orm::DatabaseConnection>,
    rule_data: web::Json<CreateRuleReq>,
) -> impl Responder {
    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_rule = ActiveModel {
        name: Set(rule_data.name.clone()),
        status: Set(rule_data.status),
        rule_id: Set(rule_data.rule_id),
        front_path: Set(rule_data.front_path.clone()),
        condition: Set(rule_data.condition.clone()),
        menu: Set(rule_data.menu),
        order: Set(rule_data.order),
        icon: Set(rule_data.icon.clone()),
        method: Set(rule_data.method.clone()),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_rule.insert(db.get_ref()).await;

    match result {
        Ok(new_rule) => HttpResponse::Ok().json(response_t(Some(200), Some(new_rule), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 修改规则
///
/// # 路径
///
/// - `id` ：整数，规则 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `rule_id` ：整数，上级菜单 ID
/// - `status` ：整数，状态，0 禁用；1 可用
/// - `name` ：字符串，菜单/权限名称
/// - `front_path` ：字符串，前端路由路径
/// - `condition` ：字符串，后端规则，由后端提供的接口别名
/// - `menu` ：整数，是否为菜单，0 否；1 是
/// - `order` ：整数，权重
/// - `icon` ：字符串，element-plus 图标
/// - `method` ：字符串，请求方式
///
/// # 响应
///
/// - 成功：状态码 200，新创建的规则
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/rules/{id}",
    request_body = UpdateRuleReq,
    responses(
        (status = 200, description = "公告更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "rules"
)]
pub async fn update_rule(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    rule_data: web::Json<UpdateRuleReq>,
) -> impl Responder {
    let rule_result = rules::Entity::find_by_id(*id).one(db.get_ref()).await;

    match rule_result {
        Ok(Some(rule)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_rule: ActiveModel = rule.into();
            updated_rule.name = rule_data.name.clone().map(Set).unwrap_or(NotSet);
            updated_rule.rule_id = rule_data.rule_id.map(Set).unwrap_or(NotSet);
            updated_rule.status = rule_data.status.map(Set).unwrap_or(NotSet);
            updated_rule.front_path = rule_data.front_path.clone().map(Set).unwrap_or(NotSet);
            updated_rule.condition = rule_data.condition.clone().map(Set).unwrap_or(NotSet);
            updated_rule.menu = rule_data.menu.map(Set).unwrap_or(NotSet);
            updated_rule.order = rule_data.order.map(Set).unwrap_or(NotSet);
            updated_rule.icon = rule_data.icon.clone().map(Set).unwrap_or(NotSet);
            updated_rule.method = rule_data.method.clone().map(Set).unwrap_or(NotSet);
            updated_rule.update_time = Set(format_time.clone());
            let result = updated_rule.update(db.get_ref()).await;
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

/// 修改规则状态
///
/// # 路径
///
/// - `id` ：整数，规则 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `status` ：整数，规则状态（必填），0 禁用；1 可用
///
/// # 响应
///
/// - 成功：状态码 200，新创建的会员等级
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    patch,
    path = "/api/rules/{id}/update_status",
    request_body = UpdateRuleStatusReq,
    responses(
        (status = 200, description = "规则状态更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "rules"
)]
pub async fn update_rule_status(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    rule_data: web::Json<UpdateRuleStatusReq>,
) -> impl Responder {
    let rule_result = rules::Entity::find_by_id(*id).one(db.get_ref()).await;

    match rule_result {
        Ok(Some(rule)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_rule: ActiveModel = rule.into();
            updated_rule.status = Set(rule_data.status.clone());
            updated_rule.update_time = Set(format_time.clone());
            let result = updated_rule.update(db.get_ref()).await;
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

/// 删除规则
///
/// # 路径
///
/// - `id` ：整数，规则 ID（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path  = "/api/rules/{id}",
    responses(
        (status = 200, description = "规则删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "rules"
)]
pub async fn delete_rule(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
) -> impl Responder {
    let result = rules::Entity::delete_by_id(*id).exec(db.get_ref()).await;
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
    path = "/api/rules",
    responses(
        (status = 200, description = "规则获取成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "rules"
)]
pub async fn get_rule_list(db: web::Data<sea_orm::DatabaseConnection>) -> impl Responder {
    let rule_result = rules::Entity::find().all(db.get_ref()).await;

    match rule_result {
        Ok(rule_list) => {
            let rule_list_len = rule_list.len() as u64;
            HttpResponse::Ok().json(response_t(
                Some(200),
                Some(response_list_t(
                    build_rule_tree(rule_list, 0),
                    rule_list_len,
                )),
                None,
            ))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
