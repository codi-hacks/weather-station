use crate::db;
use crate::error_handler::CustomError;
use crate::schema::measurements;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use crate::sensors::{SensorsModel};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "measurements"]
pub struct MeasurementsChangeset {
    pub value: BigDecimal,
    pub sensor_id: uuid::Uuid
}

#[derive(Serialize, Deserialize, Associations, Queryable, Insertable)]
#[belongs_to(SensorsModel, foreign_key = "sensor_id")]
#[table_name = "measurements"]
pub struct MeasurementsModel {
    pub id: uuid::Uuid,
    pub value: BigDecimal,
    pub sensor_id: uuid::Uuid,
    pub created_at: String
}

impl MeasurementsModel {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let measurements = measurements::table.load::<Self>(&conn)?;
        Ok(measurements)
    }

    pub fn find(id: uuid::Uuid) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let measurement = measurements::table.filter(measurements::id.eq(id)).first(&conn)?;
        Ok(measurement)
    }

    pub fn create(measurement: MeasurementsChangeset) -> Result<Self, CustomError> {
        use crate::schema::measurements::dsl::{
            value as value_column,
            created_at as created_at_column
        };
        // Get timestamp
        use chrono::{DateTime, Utc};
        use std::time::SystemTime;
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        let conn = db::connection()?;
        let measurement = MeasurementsChangeset::from(measurement);
        let measurement = diesel::insert_into(measurements::table)
            .values((
                value_column.eq(measurement.value),
                created_at_column.eq(now)
        ))
            .get_result(&conn)?;
        Ok(measurement)
    }

    pub fn update(id: uuid::Uuid, measurement: MeasurementsChangeset) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let measurement = diesel::update(measurements::table)
            .filter(measurements::id.eq(id))
            .set(measurement)
            .get_result(&conn)?;
        Ok(measurement)
    }

    pub fn delete(id: uuid::Uuid) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(measurements::table.filter(measurements::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl MeasurementsChangeset {
    fn from(measurement: MeasurementsChangeset) -> MeasurementsChangeset {
        MeasurementsChangeset {
            value: measurement.value,
            sensor_id: measurement.sensor_id
        }
    }
}
