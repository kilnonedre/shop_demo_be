use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StructUploadImageReq {
    #[schema(value_type = String, format = Binary)]
    pub img: String,
    #[serde(rename = "imageClassId")]
    pub image_class_id: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "ids": [1, 2, 3],
}))]
pub struct StructDeleteImageAllReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "测试角色名称",
}))]

pub struct StructUpdateImageReq {
    pub name: Option<String>,
}
