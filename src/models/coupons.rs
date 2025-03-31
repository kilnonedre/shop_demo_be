use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "优惠券名称",
    "type": 0,
    "value": 20.00,
    "total": 100,
    "used": 50,
    "min_price": 50.00,
    "start_time": "2022-06-11 20:14:35",
    "end_time": "2022-10-05 14:01:15",
    "order": 50,
    "status": 0,
    "desc": "描述",
}))]

pub struct CreateCouponReq {
    pub name: String,
    pub r#type: i32,
    pub value: f64,
    pub total: i32,
    pub used: i32,
    pub min_price: f64,
    pub start_time: String,
    pub end_time: String,
    pub status: i32,
    pub order: i32,
    pub desc: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "name": "优惠券名称",
    "type": 0,
    "value": 20.00,
    "total": 100,
    "used": 50,
    "min_price": 50.00,
    "start_time": "2022-06-11 20:14:35",
    "end_time": "2022-10-05 14:01:15",
    "order": 50,
    "status": 0,
    "desc": "描述",
}))]

pub struct UpdateCouponReq {
    pub name: Option<String>,
    pub r#type: Option<i32>,
    pub value: Option<f64>,
    pub total: Option<i32>,
    pub used: Option<i32>,
    pub min_price: Option<f64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub status: Option<i32>,
    pub order: Option<i32>,
    pub desc: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(example  = json!({
    "status": 1,
}))]

pub struct UpdateCouponStatusReq {
    pub status: i32,
}
