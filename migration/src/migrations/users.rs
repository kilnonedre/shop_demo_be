use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("用户 ID"),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .comment("用户名"),
                    )
                    .col(
                        ColumnDef::new(Users::Password)
                            .string()
                            .not_null()
                            .comment("密码"),
                    )
                    .col(
                        ColumnDef::new(Users::Status)
                            .integer()
                            .not_null()
                            .comment("状态：0 禁用；1 启用"),
                    )
                    .col(
                        ColumnDef::new(Users::UserLevelId)
                            .integer()
                            .not_null()
                            .comment("会员等级 ID"),
                    )
                    .col(
                        ColumnDef::new(Users::Nickname)
                            .string()
                            .null()
                            .comment("昵称"),
                    )
                    .col(
                        ColumnDef::new(Users::Phone)
                            .string()
                            .null()
                            .comment("手机号"),
                    )
                    .col(ColumnDef::new(Users::Email).string().null().comment("邮箱"))
                    .col(
                        ColumnDef::new(Users::Avatar)
                            .string()
                            .null()
                            .comment("头像"),
                    )
                    .col(
                        ColumnDef::new(Users::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Users::UpdateTime)
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
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
    Status,
    Nickname,
    Phone,
    Email,
    Avatar,
    UserLevelId,
    CreateTime,
    UpdateTime,
}
