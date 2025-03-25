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
                            .comment("菜单权限 ID"),
                    )
                    .col(
                        ColumnDef::new(Rules::Name)
                            .string()
                            .not_null()
                            .comment("名称"),
                    )
                    .col(
                        ColumnDef::new(Rules::RuleId)
                            .integer()
                            .not_null()
                            .comment("上级菜单 ID"),
                    )
                    .col(
                        ColumnDef::new(Rules::Status)
                            .integer()
                            .not_null()
                            .comment("状态：0 禁用；1 启用"),
                    )
                    .col(
                        ColumnDef::new(Rules::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Rules::UpdateTime)
                            .string()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(Rules::FrontPath)
                            .string()
                            .not_null()
                            .comment("前端路由"),
                    )
                    .col(
                        ColumnDef::new(Rules::Condition)
                            .string()
                            .not_null()
                            .comment("后端规则，由后端提供的接口别名"),
                    )
                    .col(
                        ColumnDef::new(Rules::Menu)
                            .integer()
                            .not_null()
                            .comment("是否为菜单：0 否；1 是"),
                    )
                    .col(
                        ColumnDef::new(Rules::Order)
                            .integer()
                            .not_null()
                            .comment("权重"),
                    )
                    .col(
                        ColumnDef::new(Rules::Icon)
                            .string()
                            .not_null()
                            .comment("element-plus 图标"),
                    )
                    .col(
                        ColumnDef::new(Rules::Method)
                            .string()
                            .not_null()
                            .comment("请求方式"),
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
    Name,
    RuleId,
    Status,
    CreateTime,
    UpdateTime,
    FrontPath,
    Condition,
    Menu,
    Order,
    Icon,
    Method,
}
