table! {
    measurements (id) {
        id -> Uuid,
        value -> Numeric,
        sensor_id -> Uuid,
        created_at -> Timestamp,
    }
}

table! {
    sensor_types (id) {
        id -> Uuid,
        label -> Varchar,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    stations (id) {
        id -> Uuid,
        label -> Varchar,
        key -> Varchar,
    }
}

joinable!(measurements -> sensors (sensor_id));
joinable!(sensors -> sensor_types (type_id));
joinable!(sensors -> stations (station_id));

allow_tables_to_appear_in_same_query!(
    measurements,
    sensor_types,
    sensors,
    stations,
);
