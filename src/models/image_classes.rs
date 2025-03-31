use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "测试图库名称",
    "order": 50,
}))]

pub struct CreateImageClassReq {
    pub name: String,
    pub order: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "测试图库名称",
    "order": 50,
}))]

pub struct UpdateImageClassReq {
    pub name: Option<String>,
    pub order: Option<i32>,
}
