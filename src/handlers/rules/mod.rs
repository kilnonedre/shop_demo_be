use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, Set};
use utils::insert_rule_with_child;

use crate::{
    entities::rules::{ActiveModel, Model},
    models::rules::{CreateRule, Rule},
    utils::json::read_json_from_file,
};

mod utils;

/// 创建新规则
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` : 字符串，用户名（必填）
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
    request_body = CreateRule,
    responses(
        (status = 200, description = "Rule created successfully", body = Model),
        (status = 500, description = "Internal server error")
    ),
    tag = "rules"
)]
pub async fn create_rule(
    db: web::Data<sea_orm::DatabaseConnection>,
    rule_data: web::Json<CreateRule>,
) -> impl Responder {
    let rule = ActiveModel {
        ..Default::default()
    };
    println!("{:?}", rule.rule_id);
    let result = rule.insert(db.get_ref()).await;

    match result {
        Ok(rule) => HttpResponse::Ok().json(rule),
        Err(err) => {
            log::error!("Error inserting rule: {:?}", err);
            HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err))
        }
    }
}

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
