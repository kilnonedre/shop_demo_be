use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Goods::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Goods::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null()
                            .comment("商品 ID"),
                    )
                    .col(
                        ColumnDef::new(Goods::Title)
                            .string()
                            .not_null()
                            .comment("名称"),
                    )
                    .col(
                        ColumnDef::new(Goods::CategoryId)
                            .integer()
                            .not_null()
                            .comment("商品分类 ID"),
                    )
                    .col(
                        ColumnDef::new(Goods::Cover)
                            .string()
                            .not_null()
                            .comment("封面"),
                    )
                    .col(
                        ColumnDef::new(Goods::Desc)
                            .string()
                            .not_null()
                            .comment("描述"),
                    )
                    .col(
                        ColumnDef::new(Goods::Unit)
                            .string()
                            .not_null()
                            .comment("单位"),
                    )
                    .col(
                        ColumnDef::new(Goods::Stock)
                            .integer()
                            .not_null()
                            .comment("总库存"),
                    )
                    .col(
                        ColumnDef::new(Goods::MinStock)
                            .integer()
                            .not_null()
                            .comment("库存预警"),
                    )
                    .col(
                        ColumnDef::new(Goods::Status)
                            .integer()
                            .not_null()
                            .comment("上架：0 未上架；1 已上架"),
                    )
                    .col(
                        ColumnDef::new(Goods::StockDisplay)
                            .integer()
                            .not_null()
                            .comment("库存显示：0 隐藏；1 禁用"),
                    )
                    .col(
                        ColumnDef::new(Goods::MinPrice)
                            .string()
                            .not_null()
                            .comment("最低售价"),
                    )
                    .col(
                        ColumnDef::new(Goods::MinOriPrice)
                            .string()
                            .not_null()
                            .comment("最低原价"),
                    )
                    .col(
                        ColumnDef::new(Goods::IsCheck)
                            .integer()
                            .not_null()
                            .comment("审核：0 未审核；1 同意；2 拒绝"),
                    )
                    .col(
                        ColumnDef::new(Goods::CreateTime)
                            .string()
                            .not_null()
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(Goods::UpdateTime)
                            .string()
                            .not_null()
                            .comment("更新时间"),
                    )
                    .col(
                        ColumnDef::new(Goods::DeleteTime)
                            .string()
                            .null()
                            .comment("删除时间"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Goods::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Goods {
    Table,
    Id,
    Title,
    CategoryId,
    Cover,
    Desc,
    Unit,
    Stock,
    MinStock,
    Status,
    StockDisplay,
    MinPrice,
    MinOriPrice,
    IsCheck,
    CreateTime,
    UpdateTime,
    DeleteTime,
}
