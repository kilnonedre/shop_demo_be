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
