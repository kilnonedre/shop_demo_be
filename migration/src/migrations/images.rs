use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Images::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Images::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("图片 ID"),
                    )
                    .col(
                        ColumnDef::new(Images::Name)
                            .string()
                            .not_null()
                            .comment("名称"),
                    )
                    .col(
                        ColumnDef::new(Images::Url)
                            .string()
                            .not_null()
                            .comment("图片地址"),
                    )
                    .col(
                        ColumnDef::new(Images::Path)
                            .string()
                            .not_null()
                            .comment("图片路径"),
                    )
                    .col(
                        ColumnDef::new(Images::ImageClassId)
                            .integer()
                            .not_null()
                            .comment("图库 ID"),
                    )
                    .col(
                        ColumnDef::new(Images::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Images::UpdateTime)
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
            .drop_table(Table::drop().table(Images::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Images {
    Table,
    Id,
    Name,
    Url,
    Path,
    ImageClassId,
    CreateTime,
    UpdateTime,
}
