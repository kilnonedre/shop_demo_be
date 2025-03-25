use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema, Clone, Serialize)]

pub struct StructRule {
    pub id: i32,
    pub name: String,
    pub rule_id: i32,
    pub status: i32,
    pub create_time: String,
    pub update_time: String,
    pub front_path: String,
    pub condition: String,
    pub menu: i32,
    pub order: i32,
    pub icon: String,
    pub method: String,
    pub child: Vec<StructRule>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "后台面板",
    "rule_id": 0,
    "status": 1,
    "front_path": "",
    "condition": "",
    "menu": 1,
    "order": 1,
    "icon": "help",
    "method": "GET",
}))]

pub struct StructCreateRuleReq {
    pub name: String,
    pub rule_id: i32,
    pub status: i32,
    pub front_path: String,
    pub condition: String,
    pub menu: i32,
    pub order: i32,
    pub icon: String,
    pub method: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "后台面板",
    "rule_id": 0,
    "status": 1,
    "front_path": "",
    "condition": "",
    "menu": 1,
    "order": 1,
    "icon": "help",
    "method": "GET",
}))]

pub struct StructUpdateRuleReq {
    pub name: Option<String>,
    pub rule_id: Option<i32>,
    pub status: Option<i32>,
    pub front_path: Option<String>,
    pub condition: Option<String>,
    pub menu: Option<i32>,
    pub order: Option<i32>,
    pub icon: Option<String>,
    pub method: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "status": 1,
}))]

pub struct StructUpdateRuleStatusReq {
    pub status: i32,
}
