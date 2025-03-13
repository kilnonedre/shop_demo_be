use actix_web::{web, Scope};
use manager::build_admin_manager_router;

mod manager;

pub fn build_admin_router() -> Scope {
    web::scope("/admins").service(build_admin_manager_router())
}
