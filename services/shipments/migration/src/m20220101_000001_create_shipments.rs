use sea_orm_migration::prelude::*;
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::sea_orm::ConnectionTrait;
use crate::sea_orm::{DbBackend, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .create_type(
                    Type::create()
                            .as_enum(ShippingStatus::Type)
                            .values(vec![ShippingStatus::Created, ShippingStatus::Packaging, ShippingStatus::Shipping, ShippingStatus::Delivered])
                            .to_owned(),
                ).await?
        }

        manager
            .create_table(
                Table::create()
                    .table(Shipment::Table)
                    .col(
                        ColumnDef::new(Shipment::Id)
                            .uuid()
                            .primary_key()
                            .not_null()
                            .extra("DEFAULT public.uuid_generate_v4()".into()),
                    )
                    .col(
                        ColumnDef::new(Shipment::OrderId)
                            .big_integer()
                            .unique_key()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Shipment::ShippingStatus)
                            .enumeration(
                                Alias::new("status"),
                                [
                                    Alias::new("created"),
                                    Alias::new("packaging"),
                                    Alias::new("shipping"),
                                    Alias::new("delivered")
                                ]
                            )
                            .default("created")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Shipment::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .col(
                        ColumnDef::new(Shipment::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Shipment::Table).to_owned())
            .await?;

        if manager.get_database_backend() == DbBackend::Postgres {
            manager
                .drop_type(Type::drop().name(ShippingStatus::Type).to_owned())
                .await?;
        }

        Ok(())
    }
}

#[derive(Iden)]
enum Shipment {
    Table,
    Id,
    OrderId,
    ShippingStatus,
    UpdatedAt,
    CreatedAt
}

#[derive(Iden)]
enum ShippingStatus {
    #[iden = "status"]
    Type,
    Created,
    Packaging,
    Shipping,
    Delivered
}