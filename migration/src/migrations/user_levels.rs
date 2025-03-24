use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserLevels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLevels::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("会员等级 ID"),
                    )
                    .col(
                        ColumnDef::new(UserLevels::Name)
                            .string()
                            .not_null()
                            .comment("会员等级名称"),
                    )
                    .col(
                        ColumnDef::new(UserLevels::Level)
                            .integer()
                            .not_null()
                            .comment("等级"),
                    )
                    .col(
                        ColumnDef::new(UserLevels::Status)
                            .integer()
                            .not_null()
                            .comment("状态：0 禁用；1 启用"),
                    )
                    .col(
                        ColumnDef::new(UserLevels::Discount)
                            .integer()
                            .not_null()
                            .comment("折扣率 %"),
                    )
                    .col(
                        ColumnDef::new(UserLevels::MaxPrice)
                            .integer()
                            .not_null()
                            .comment("累计消费金额"),
                    )
                    .col(
                        ColumnDef::new(UserLevels::MaxTime)
                            .integer()
                            .not_null()
                            .comment("累计消费次数"),
                    )
                    .col(
                        ColumnDef::new(UserLevels::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserLevels::UpdateTime)
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
            .drop_table(Table::drop().table(UserLevels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserLevels {
    Table,
    Id,
    Name,
    Level,
    Status,
    Discount,
    MaxPrice,
    MaxTime,
    CreateTime,
    UpdateTime,
}
