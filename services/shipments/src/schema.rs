// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    flyway_schema_history (installed_rank) {
        installed_rank -> Int4,
        version -> Nullable<Varchar>,
        description -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        script -> Varchar,
        checksum -> Nullable<Int4>,
        installed_by -> Varchar,
        installed_on -> Timestamp,
        execution_time -> Int4,
        success -> Bool,
    }
}

diesel::table! {
    refinery_schema_history (version) {
        version -> Int4,
        name -> Nullable<Varchar>,
        applied_on -> Nullable<Varchar>,
        checksum -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    flyway_schema_history,
    refinery_schema_history,
    users,
);
