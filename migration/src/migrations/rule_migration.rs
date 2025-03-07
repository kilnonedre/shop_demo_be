use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Rules::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rules::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("用户 ID"),
                    )
                    .col(
                        ColumnDef::new(Rules::RuleId)
                            .integer()
                            .not_null()
                            .comment(""),
                    )
                    .col(
                        ColumnDef::new(Rules::Status)
                            .integer()
                            .not_null()
                            .comment("状态"),
                    )
                    .col(
                        ColumnDef::new(Rules::CreateTime)
                            .date_time()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Rules::UpdateTime)
                            .date_time()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(Rules::Name)
                            .string()
                            .not_null()
                            .comment("名称"),
                    )
                    .col(
                        ColumnDef::new(Rules::Desc)
                            .string()
                            .not_null()
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(Rules::FrontPath)
                            .string()
                            .null()
                            .comment("前端路由"),
                    )
                    .col(ColumnDef::new(Rules::Condition).string().null().comment(""))
                    .col(ColumnDef::new(Rules::Menu).integer().not_null().comment(""))
                    .col(ColumnDef::new(Rules::Order).integer().null().comment(""))
                    .col(ColumnDef::new(Rules::Icon).string().null().comment("图标"))
                    .col(
                        ColumnDef::new(Rules::Method)
                            .string()
                            .null()
                            .comment("方法"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rules::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Rules {
    Table,
    Id,
    RuleId,
    Status,
    CreateTime,
    UpdateTime,
    Name,
    Desc,
    FrontPath,
    Condition,
    Menu,
    Order,
    Icon,
    Method,
}
