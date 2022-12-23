use std::env;
use sea_orm_migration::prelude::*;
use migration::sea_orm::{ConnectionTrait, Database, Statement};

#[async_std::main]
async fn main() {
    let user = env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
    let password = env::var("POSTGRES_PASSWORD").unwrap_or(String::from("demo"));
    let db = env::var("POSTGRES_DB").unwrap_or("demo".parse().unwrap());
    let db_host = env::var("POSTGRES_SERVICE_HOST").unwrap_or("localhost".parse().unwrap());
    let db_port = env::var("POSTGRES_SERVICE_PORT").unwrap_or("5432".parse().unwrap());

    let db = Database::connect(format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, db_host, db_port, db
    )).await.expect("Can't connect to database");

    let schema = r#"
                CREATE SCHEMA shipments;
            "#;
    let stmt = Statement::from_string(db.get_database_backend(), schema.to_owned());

    db.execute(stmt).await.expect("Schema creation failed");

    let uuid = r#"
                CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
            "#;
    let stmt = Statement::from_string(db.get_database_backend(), uuid.to_owned());
    db.execute(stmt).await.expect("UUID extension creation failed");

    cli::run_cli(migration::Migrator).await;
}
