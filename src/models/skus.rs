use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "规格名称",
    "status": 0,
    "order": 50,
    "default": "规格1,规格2",
}))]

pub struct CreateSkuReq {
    pub name: String,
    pub status: i32,
    pub order: i32,
    pub default: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "规格名称",
    "status": 0,
    "order": 50,
    "default": "规格1,规格2",
}))]

pub struct UpdateSkuReq {
    pub name: Option<String>,
    pub status: Option<i32>,
    pub order: Option<i32>,
    pub default: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "status": 1,
}))]

pub struct UpdateSkuStatusReq {
    pub status: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "ids": [1, 2],
}))]

pub struct DeleteSkuAllReq {
    pub ids: Vec<i32>,
}
