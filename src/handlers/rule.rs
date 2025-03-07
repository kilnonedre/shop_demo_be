use actix_web::{web, HttpResponse, Responder};
// use sea_orm::{prelude::DateTime, ActiveModelTrait, Set};
use sea_orm::{prelude::DateTime, ActiveModelTrait, Set};
use serde::Deserialize;
use utoipa::{schema, ToSchema};

use crate::entity::rules::{ActiveModel, Model};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "rule_id": 0,
    "status": 1,
    "create_time": "2023-09-09T15:53:00",
    "update_time": "2023-09-09T15:53:00",
    "name": "后台面板",
    "desc": "index",
    "front_path": null,
    "condition": null,
    "menu": 1,
    "order": 1,
    "icon": "help",
    "method": "GET",
}))]

pub struct CreateRule {
    pub rule_id: i32,
    pub status: i32,
    #[schema(value_type = String)]
    pub create_time: DateTime,
    #[schema(value_type = String)]
    pub update_time: DateTime,
    pub name: String,
    pub desc: String,
    pub front_path: Option<String>,
    pub condition: Option<String>,
    pub menu: i32,
    pub order: Option<i32>,
    pub icon: Option<String>,
    pub method: Option<String>,
}
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
    )
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
