use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "title": "测试标题",
    "content": "测试内容",
}))]

pub struct StructCreateNotice {
    pub title: String,
    pub content: String,
}
