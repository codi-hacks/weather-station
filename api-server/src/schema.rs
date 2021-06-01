table! {
    employees (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        department -> Varchar,
        salary -> Int4,
        age -> Int4,
    }
}

table! {
    measurements (id) {
        id -> Uuid,
        value -> Int4,
        sensor_id -> Uuid,
        created_at -> Varchar,
    }
}

table! {
    sensor_types (id) {
        id -> Uuid,
        label -> Numeric,
        description -> Varchar,
    }
}

table! {
    sensors (id) {
        id -> Uuid,
        alias -> Varchar,
        label -> Varchar,
        type_id -> Uuid,
        station_id -> Uuid,
        created_at -> Varchar,
        updated_at -> Varchar,
    }
}

table! {
    stations (id) {
        id -> Uuid,
        label -> Varchar,
        key -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    measurements,
    sensor_types,
    sensors,
    stations,
);
