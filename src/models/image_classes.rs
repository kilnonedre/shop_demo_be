use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "测试图库名称",
    "order": 50,
}))]

pub struct StructCreateImageClassReq {
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "测试图库名称",
    "order": 50,
}))]

pub struct StructUpdateImageClassReq {
    pub name: Option<String>,
    pub order: Option<i32>,
}
