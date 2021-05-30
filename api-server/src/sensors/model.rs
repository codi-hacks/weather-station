use crate::db;
use crate::error_handler::CustomError;
use crate::schema::sensors;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "sensors"]
pub struct SensorsChangeset {
    pub alias: String,
    pub label: String
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "sensors"]
pub struct SensorsModel {
    pub id: uuid::Uuid,
    pub alias: String,
    pub label: String,
    pub type_id: uuid::Uuid,
    pub station_id: uuid::Uuid,
    pub created_at: String,
    pub updated_at: String
}

impl SensorsModel {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let sensors = sensors::table.load::<Self>(&conn)?;
        Ok(sensors)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let sensor = sensors::table.filter(sensors::id.eq(id)).first(&conn)?;
        Ok(sensor)
    }

    pub fn create(sensor: SensorsChangeset) -> Result<Self, CustomError> {
        use crate::schema::sensors::dsl::{
            alias as alias_column,
            label as label_column,
            created_at as created_at_column,
            updated_at as updated_at_column
        };
        // Get timestamp
        use chrono::{DateTime, Utc};
        use std::time::SystemTime;
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        let conn = db::connection()?;
        let sensor = SensorsChangeset::from(sensor);
        let sensor = diesel::insert_into(sensors::table)
            .values((
                alias_column.eq(sensor.alias),
                label_column.eq(sensor.label),
                created_at_column.eq(now.clone()),
                updated_at_column.eq(now)
            ))
            .get_result(&conn)?;
        Ok(sensor)
    }

    pub fn update(id: uuid::Uuid, sensor: SensorsChangeset) -> Result<Self, CustomError> {
        use crate::schema::sensors::dsl::{
            alias as alias_column,
            label as label_column,
            updated_at as updated_at_column
        };
        // Get timestamp
        use chrono::{DateTime, Utc};
        use std::time::SystemTime;
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        let conn = db::connection()?;
        let sensor = diesel::update(sensors::table)
            .filter(sensors::id.eq(id))
            .set((
                alias_column.eq(sensor.alias),
                label_column.eq(sensor.label),
                updated_at_column.eq(now)
            ))
            .get_result(&conn)?;
        Ok(sensor)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(sensors::table.filter(sensors::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl SensorsChangeset {
    fn from(sensor: SensorsChangeset) -> SensorsChangeset {
        SensorsChangeset {
            alias: sensor.alias,
            label: sensor.label
        }
    }
}
