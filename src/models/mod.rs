use serde::Deserialize;

pub mod admins;
pub mod notices;
pub mod roles;
pub mod rules;

#[derive(Deserialize)]
pub struct StructPagination {
    pub page: Option<u64>,
    pub size: Option<u64>,
}
