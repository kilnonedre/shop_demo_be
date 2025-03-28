use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

pub mod admins;
pub mod coupons;
pub mod images;
pub mod notices;
pub mod roles;
pub mod rules;
pub mod skus;
pub mod user_levels;
pub mod users;

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct StructPagination {
    #[param(style = Form, allow_reserved, example = 1)]
    pub page: u64,
    #[param(style = Form, allow_reserved, example = 10)]
    pub size: u64,
}
