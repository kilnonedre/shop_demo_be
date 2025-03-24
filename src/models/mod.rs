use serde::Deserialize;

pub mod admins;
pub mod coupons;
pub mod notices;
pub mod roles;
pub mod rules;
pub mod skus;
pub mod user_levels;
pub mod users;

#[derive(Deserialize)]
pub struct StructPagination {
    pub page: Option<u64>,
    pub size: Option<u64>,
}
