use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, Error, PgPool, query};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipment {
    pub id: Uuid,
    pub order_id: i32,
    pub status: Status,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "status")]
pub enum Status {
    CREATED,
    PACKAGING,
    SHIPPING,
    DELIVERED,
}

impl Shipment {
    pub async fn create(db: &PgPool, order_id: i32) -> Result<Self, Error> {
        // let shipment_result = query_as!(
        //     Shipment,
        //     r#"INSERT INTO shipments.shipment (order_id) VALUES ($1) RETURNING id, order_id, status as "status: Status", updated_at, created_at"#,
        //     order_id,
        // ).fetch_one(db).await;
        //
        // shipment_result
        Err(Error::RowNotFound)
    }

    pub async fn get_all(db: &PgPool) -> Result<Vec<Self>, Error> {
        // let shipments_result = query_as!(
        //     Shipment,
        //     r#"SELECT id, order_id, status as "status: Status", updated_at, created_at FROM shipments.shipment"#
        // ).fetch_all(db).await;
        //
        // shipments_result
        Err(Error::RowNotFound)
    }

    pub async fn get(db: &PgPool, id: Uuid) -> Result<Self, Error> {
        // let shipment_result = query_as!(
        //     Shipment,
        //     r#"SELECT id, order_id, status as "status: Status", updated_at, created_at FROM shipments.shipment WHERE id = $1"#,
        //     id
        // ).fetch_one(db).await;
        //
        // shipment_result
        Err(Error::RowNotFound)
    }

    pub async fn update_status(self, db: &PgPool, status: Status) -> Result<PgQueryResult, Error> {
        Err(Error::RowNotFound)
    }

    pub async fn delete(self, db: &PgPool) -> Result<PgQueryResult, Error> {
        // let delete_result = query!(
        //     r#"DELETE FROM shipments.shipment WHERE id = $1"#,
        //     self.id
        // ).execute(db).await;
        //
        // delete_result
        Err(Error::RowNotFound)
    }
}
