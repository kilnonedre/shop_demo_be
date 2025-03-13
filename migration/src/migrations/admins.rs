use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Admins::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Admins::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("管理员 ID"),
                    )
                    .col(
                        ColumnDef::new(Admins::Username)
                            .string()
                            .not_null()
                            .comment("用户名"),
                    )
                    .col(
                        ColumnDef::new(Admins::Password)
                            .string()
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(Admins::RoleId)
                            .integer()
                            .not_null()
                            .comment("角色 ID"),
                    )
                    .col(
                        ColumnDef::new(Admins::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Admins::UpdateTime)
                            .string()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(Admins::Status)
                            .integer()
                            .not_null()
                            .comment("状态：0 禁用；1 启用"),
                    )
                    .col(
                        ColumnDef::new(Admins::Avatar)
                            .string()
                            .null()
                            .comment("头像"),
                    )
                    .col(
                        ColumnDef::new(Admins::IsSuper)
                            .integer()
                            .not_null()
                            .comment("是否为超级管理员：0 否；1 是"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Admins::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Admins {
    Table,
    Id,
    Username,
    Password,
    RoleId,
    CreateTime,
    UpdateTime,
    Status,
    Avatar,
    IsSuper,
}
