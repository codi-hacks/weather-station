use crate::error_handler::CustomError;
use crate::schema::measurements;
use crate::sensors::{Sensor, SensorsModel};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::DbConnection;

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
    pub fn create(sensor_id: Uuid, value: BigDecimal, conn: &DbConnection) -> Result<Self, CustomError> {
        let measurement = diesel::insert_into(measurements::table)
            .values((
                measurements::value.eq(value),
                measurements::sensor_id.eq(sensor_id)
            ))
            .get_result(conn)?;
        SensorsModel::touch(sensor_id, conn)?;
        Ok(measurement)
    }

    pub fn delete_by_sensor(sensor: &Sensor, conn: &DbConnection) -> Result<usize, CustomError> {
        let sensor = SensorsModel {
            id: sensor.id,
            alias: sensor.alias.clone(),
            label: sensor.label.clone(),
            type_id: sensor.type_id,
            station_id: sensor.station_id,
            created_at: sensor.created_at,
            updated_at: sensor.updated_at
        };
        Ok(diesel::delete(MeasurementsModel::belonging_to(&sensor)).execute(conn)?)
    }
}
