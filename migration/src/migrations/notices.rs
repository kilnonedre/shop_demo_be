use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notices::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("公告 ID"),
                    )
                    .col(
                        ColumnDef::new(Notices::Title)
                            .string()
                            .not_null()
                            .comment("标题"),
                    )
                    .col(
                        ColumnDef::new(Notices::Content)
                            .string()
                            .not_null()
                            .comment("内容"),
                    )
                    .col(
                        ColumnDef::new(Notices::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Notices::UpdateTime)
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
            .drop_table(Table::drop().table(Notices::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Notices {
    Table,
    Id,
    Title,
    Content,
    CreateTime,
    UpdateTime,
}
