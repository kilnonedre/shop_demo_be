use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "title": "商品名称",
    "category_id": 0,
    "cover": "http://...png",
    "desc": "描述",
    "unit": "kg",
    "stock": 200,
    "min_stock": 10,
    "status": 1,
    "stock_display": 1,
    "min_price": "1.00",
    "min_ori_price": "100.00",
    "is_check": 0,
}))]

pub struct CreateGoodReq {
    pub title: String,
    pub category_id: i32,
    pub cover: String,
    pub desc: String,
    pub unit: String,
    pub stock: i32,
    pub min_stock: i32,
    pub status: i32,
    pub stock_display: i32,
    pub min_price: String,
    pub min_ori_price: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "id": 1,
    "title": "商品名称",
    "category_id": 0,
    "cover": "http://...png",
    "desc": "描述",
    "unit": "kg",
    "stock": 200,
    "min_stock": 10,
    "status": 1,
    "stock_display": 1,
    "min_price": "1.00",
    "min_ori_price": "100.00",
    "is_check": 0,
    "create_time": "2022-06-17 19:57:32",
    "update_time": "2022-06-17 19:57:32",
}))]

pub struct CreateGoodRes {
    pub id: i32,
    pub title: String,
    pub category_id: i32,
    pub cover: String,
    pub desc: String,
    pub unit: String,
    pub stock: i32,
    pub min_stock: i32,
    pub status: i32,
    pub stock_display: i32,
    pub min_price: String,
    pub min_ori_price: String,
    pub is_check: i32,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "title": "商品名称",
    "category_id": 0,
    "cover": "http://...png",
    "desc": "描述",
    "unit": "kg",
    "stock": 200,
    "min_stock": 10,
    "status": 1,
    "stock_display": 1,
    "min_price": "1.00",
    "min_ori_price": "100.00",
}))]

pub struct UpdateGoodReq {
    pub title: Option<String>,
    pub category_id: Option<i32>,
    pub cover: Option<String>,
    pub desc: Option<String>,
    pub unit: Option<String>,
    pub stock: Option<i32>,
    pub min_stock: Option<i32>,
    pub status: Option<i32>,
    pub stock_display: Option<i32>,
    pub min_price: Option<String>,
    pub min_ori_price: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "ids": [0, 1, 2],
    "status": 0,
}))]

pub struct BatchUpdateStatusReq {
    pub ids: Vec<i32>,
    pub status: i32,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]

pub struct GetGoodListReq {
    #[param(style = Form, allow_reserved, example = 1)]
    pub page: u64,
    #[param(style = Form, allow_reserved, example = 10)]
    pub size: u64,
    pub tab: Option<String>,
    pub title: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "ids": [0, 1, 2],
}))]

pub struct BatchSoftDeleteGoodReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "ids": [0, 1, 2],
}))]

pub struct BatchRestoreGoodReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "ids": [0, 1, 2],
}))]

pub struct BatchDeleteGoodReq {
    pub ids: Vec<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "is_check": 1,
}))]

pub struct UpdateGoodIsCheckReq {
    pub is_check: i32,
}
