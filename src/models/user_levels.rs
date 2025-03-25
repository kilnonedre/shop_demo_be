use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "会员等级名称",
    "status": 0,
    "level": 100,
    "discount": 10,
    "max_price": 1000,
    "max_time": 500,
}))]

pub struct StructCreateUserLevelReq {
    pub name: String,
    pub level: i32,
    pub status: i32,
    pub discount: i32,
    pub max_price: i32,
    pub max_time: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "会员等级名称",
    "status": 0,
    "level": 100,
    "discount": 10,
    "max_price": 1000,
    "max_time": 500,
}))]

pub struct StructUpdateUserLevelReq {
    pub name: Option<String>,
    pub level: Option<i32>,
    pub status: Option<i32>,
    pub discount: Option<i32>,
    pub max_price: Option<i32>,
    pub max_time: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "status": 1,
}))]

pub struct StructUpdateUserLevelStatusReq {
    pub status: i32,
}
