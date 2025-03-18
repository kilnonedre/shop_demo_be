use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "测试角色名称",
    "desc": "测试角色描述",
    "status": 1,
}))]

pub struct StructCreateRoleReq {
    pub name: String,
    pub desc: String,
    pub status: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "测试角色名称",
    "desc": "测试角色描述",
    "status": 1,
}))]

pub struct StructUpdateRoleReq {
    pub name: Option<String>,
    pub desc: Option<String>,
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "status": 1,
}))]

pub struct StructUpdateRoleStatusReq {
    pub status: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "rule_ids": [ 5, 10, 174, 175, 176 ],
}))]

pub struct StructUpdateRoleRuleIdsReq {
    pub rule_ids: Vec<i32>,
}
