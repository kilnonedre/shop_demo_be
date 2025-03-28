use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ImageClasses::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ImageClasses::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("图库 ID"),
                    )
                    .col(
                        ColumnDef::new(ImageClasses::Name)
                            .string()
                            .not_null()
                            .comment("名称"),
                    )
                    .col(
                        ColumnDef::new(ImageClasses::Order)
                            .integer()
                            .not_null()
                            .comment("权重"),
                    )
                    .col(
                        ColumnDef::new(ImageClasses::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(ImageClasses::UpdateTime)
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
            .drop_table(Table::drop().table(ImageClasses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ImageClasses {
    Table,
    Id,
    Name,
    Order,
    CreateTime,
    UpdateTime,
}
