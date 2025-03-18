//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, ToSchema, Serialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub status: i32,
    pub desc: String,
    pub rule_ids: Option<Vec<i32>>,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
