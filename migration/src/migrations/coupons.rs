use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Coupons::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Coupons::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("优惠券 ID"),
                    )
                    .col(
                        ColumnDef::new(Coupons::Name)
                            .string()
                            .not_null()
                            .comment("优惠券名称"),
                    )
                    .col(
                        ColumnDef::new(Coupons::Type)
                            .integer()
                            .not_null()
                            .comment("类型：0 满减；1 折扣"),
                    )
                    .col(
                        ColumnDef::new(Coupons::Value)
                            .double()
                            .not_null()
                            .comment("面值"),
                    )
                    .col(
                        ColumnDef::new(Coupons::Total)
                            .integer()
                            .not_null()
                            .comment("发行量"),
                    )
                    .col(
                        ColumnDef::new(Coupons::Used)
                            .integer()
                            .not_null()
                            .comment("使用量"),
                    )
                    .col(
                        ColumnDef::new(Coupons::MinPrice)
                            .double()
                            .not_null()
                            .comment("最低使用价格"),
                    )
                    .col(
                        ColumnDef::new(Coupons::StartTime)
                            .string()
                            .not_null()
                            .comment("开始时间"),
                    )
                    .col(
                        ColumnDef::new(Coupons::EndTime)
                            .string()
                            .not_null()
                            .comment("结束时间"),
                    )
                    .col(
                        ColumnDef::new(Coupons::Status)
                            .integer()
                            .not_null()
                            .comment("状态：0 禁用；1 启用"),
                    )
                    .col(
                        ColumnDef::new(Coupons::Order)
                            .integer()
                            .not_null()
                            .comment("权重"),
                    )
                    .col(
                        ColumnDef::new(Coupons::Desc)
                            .string()
                            .not_null()
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(Coupons::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Coupons::UpdateTime)
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
            .drop_table(Table::drop().table(Coupons::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Coupons {
    Table,
    Id,
    Name,
    Type,
    Value,
    Total,
    Used,
    MinPrice,
    StartTime,
    EndTime,
    Status,
    Order,
    Desc,
    CreateTime,
    UpdateTime,
}
