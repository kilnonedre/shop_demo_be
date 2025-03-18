use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("角色 ID"),
                    )
                    .col(
                        ColumnDef::new(Roles::Name)
                            .string()
                            .not_null()
                            .comment("角色名称"),
                    )
                    .col(
                        ColumnDef::new(Roles::Status)
                            .integer()
                            .not_null()
                            .comment("状态：0 禁用；1 启用"),
                    )
                    .col(
                        ColumnDef::new(Roles::Desc)
                            .string()
                            .not_null()
                            .comment("角色描述"),
                    )
                    .col(
                        ColumnDef::new(Roles::RuleIds)
                            .array(ColumnType::Integer)
                            .null()
                            .comment("权限 ID 列表"),
                    )
                    .col(
                        ColumnDef::new(Roles::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Roles::UpdateTime)
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
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    Name,
    Status,
    Desc,
    RuleIds,
    CreateTime,
    UpdateTime,
}
