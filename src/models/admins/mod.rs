use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "username": "admin2",
    "role_id": 38,
    "status": 1,
    "avatar": null,
    "password": "xxxxxxxxxx",
}))]

pub struct StructCreateAdmin {
    pub username: String,
    pub password: String,
    pub role_id: i32,
    pub status: i32,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "status": 1,
}))]

pub struct StructUpdateAdminStatus {
    pub status: i32,
}
