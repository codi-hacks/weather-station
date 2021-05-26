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
    stations (id) {
        id -> Uuid,
        label -> Varchar,
        key -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    employees,
    stations,
);
