use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseT<T> {
    code: i16,
    data: Option<T>,
    msg: String,
}

pub fn response_t<T>(code: Option<i16>, data: Option<T>, msg: Option<String>) -> ResponseT<T> {
    ResponseT {
        code: code.unwrap_or(200),
        data: data,
        msg: msg.unwrap_or(String::from("ok")),
    }
}

#[derive(Serialize, ToSchema)]
pub struct ResponseListT<T> {
    list: Vec<T>,
    total_count: u64,
}

pub fn response_list_t<T>(list: Vec<T>, total_count: u64) -> ResponseListT<T> {
    ResponseListT {
        list: list,
        total_count: total_count,
    }
}
