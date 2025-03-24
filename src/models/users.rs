use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "username": "测试",
    "password": "test_test_test",
    "status": 1,
    "user_level_id": 1,
    "nickname": null,
    "phone": null,
    "email": null,
    "avatar": null,
}))]

pub struct StructCreateUserReq {
    pub username: String,
    pub password: String,
    pub status: i32,
    pub user_level_id: i32,
    pub nickname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "username": "测试",
    "password": "test_test_test",
    "status": 1,
    "user_level_id": 1,
    "nickname": null,
    "phone": null,
    "email": null,
    "avatar": null,
}))]

pub struct StructUpdateUserReq {
    pub username: Option<String>,
    pub password: Option<String>,
    pub status: Option<i32>,
    pub user_level_id: Option<i32>,
    pub nickname: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "status": 1,
}))]

pub struct StructUpdateUserStatusReq {
    pub status: i32,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct StructGetUserListReq {
    #[param(style = Form, allow_reserved, example = 1)]
    pub page: u64,
    #[param(style = Form, allow_reserved, example = 10)]
    pub size: u64,
    pub keyword: Option<String>,
    pub user_level_id: Option<i32>,
}
