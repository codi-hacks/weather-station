use crate::db;
use crate::error_handler::CustomError;
use crate::schema::measurements;
use crate::sensors::{Sensor, SensorsModel};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(AsChangeset, Associations, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[belongs_to(SensorsModel, foreign_key = "sensor_id")]
#[table_name = "measurements"]
pub struct MeasurementsModel {
    pub id: Uuid,
    pub value: BigDecimal,
    // Measurements only come back inside sensor data so this
    // information is redundant from a JSON perspective.
    #[serde(skip_serializing)]
    pub sensor_id: Uuid,
    pub created_at: NaiveDateTime
}

impl MeasurementsModel {
    pub fn create(sensor_id: Uuid, value: BigDecimal) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let measurement = diesel::insert_into(measurements::table)
            .values((
                measurements::value.eq(value),
                measurements::sensor_id.eq(sensor_id)
            ))
            .get_result(&conn)?;
        SensorsModel::touch(sensor_id)?;
        Ok(measurement)
    }
    
    // pub fn delete(id: Uuid) -> Result<usize, CustomError> {
    //     let conn = db::connection()?;
    //     let res = diesel::delete(measurements::table.filter(measurements::id.eq(id))).execute(&conn)?;
    //     Ok(res)
    // }
    
    pub fn delete_by_sensor(sensor: Sensor) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let sensor = SensorsModel {
            id: sensor.id,
            alias: sensor.alias,
            label: sensor.label,
            type_id: sensor.type_id,
            station_id: sensor.station_id,
            created_at: sensor.created_at,
            updated_at: sensor.updated_at
        };
        Ok(diesel::delete(MeasurementsModel::belonging_to(&sensor)).execute(&conn)?)
    }
}
