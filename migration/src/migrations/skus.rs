use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Skus::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Skus::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("规格 ID"),
                    )
                    .col(
                        ColumnDef::new(Skus::Name)
                            .string()
                            .not_null()
                            .comment("规格名称"),
                    )
                    .col(
                        ColumnDef::new(Skus::Type)
                            .integer()
                            .not_null()
                            .comment("未知"),
                    )
                    .col(
                        ColumnDef::new(Skus::Status)
                            .integer()
                            .not_null()
                            .comment("状态：0 禁用；1 启用"),
                    )
                    .col(
                        ColumnDef::new(Skus::Order)
                            .integer()
                            .not_null()
                            .comment("权重"),
                    )
                    .col(
                        ColumnDef::new(Skus::Default)
                            .string()
                            .not_null()
                            .comment("规格值"),
                    )
                    .col(
                        ColumnDef::new(Skus::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Skus::UpdateTime)
                            .string()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Skus::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Skus {
    Table,
    Id,
    Name,
    Type,
    Status,
    Order,
    Default,
    CreateTime,
    UpdateTime,
}
