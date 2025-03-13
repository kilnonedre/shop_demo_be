use sea_orm::prelude::DateTime;
use serde::Deserialize;
use utoipa::ToSchema;

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

#[derive(Debug, Deserialize, ToSchema)]

pub struct Rule {
    pub id: i32,
    pub rule_id: i32,
    pub status: i32,
    pub create_time: String,
    pub update_time: String,
    pub name: String,
    pub desc: Option<String>,
    pub frontpath: Option<String>,
    pub condition: Option<String>,
    pub menu: i32,
    pub order: Option<i32>,
    pub icon: Option<String>,
    pub method: Option<String>,
    pub child: Vec<Rule>,
}
