use crate::db;
use crate::error_handler::CustomError;
use crate::schema::sensors;
use crate::sensor_types::{SensorTypesModel};
use crate::stations::{StationsModel};
use crate::measurements::MeasurementsModel;

use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid:: Uuid;

#[derive(Deserialize, Serialize)] 
pub struct Sensor {
    pub id: uuid::Uuid,
    pub alias: String,
    pub label: String,
    pub type_id: uuid::Uuid,
    pub station_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub measurements: Vec<MeasurementsModel>
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "sensors"]
pub struct SensorsChangeset {
    pub alias: String,
    pub label: String,
    pub type_id: uuid::Uuid,
    pub station_id: uuid::Uuid
}


#[derive(AsChangeset, Associations, Clone, Deserialize, Identifiable, Insertable, Queryable, Serialize)] 
#[belongs_to(SensorTypesModel, foreign_key = "type_id")]
#[belongs_to(StationsModel, foreign_key = "station_id")]
#[table_name = "sensors"]
pub struct SensorsModel {
    pub id: uuid::Uuid,
    pub alias: String,
    pub label: String,
    pub type_id: Uuid,
    pub station_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl SensorsModel {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let sensors = sensors::table.load::<Self>(&conn)?;
        Ok(sensors)
    }

    pub fn find(id: uuid::Uuid) -> Result<Sensor, CustomError> {
        let conn = db::connection()?;
        let sensor: Self = sensors::table.filter(sensors::id.eq(id)).first(&conn)?;
        let measurements: Vec<MeasurementsModel> = MeasurementsModel::belonging_to(&sensor).load(&conn)?;
        Ok(Sensor {
            id:             sensor.id,
            alias:          sensor.alias,
            label:          sensor.label,
            type_id:        sensor.type_id,
            station_id:     sensor.station_id,
            created_at:     sensor.created_at,
            updated_at:     sensor.updated_at,
            measurements
        })
    }

    pub fn create(sensor: SensorsChangeset) -> Result<Self, CustomError> {
        use crate::schema::sensors::dsl::{
            alias as alias_column,
            label as label_column,
            type_id as type_id_column,
            station_id as station_id_column,
        };

        let conn = db::connection()?;
        let sensor = SensorsChangeset::from(sensor);
        let sensor = diesel::insert_into(sensors::table)
            .values((
                alias_column.eq(sensor.alias),
                label_column.eq(sensor.label),
                type_id_column.eq(sensor.type_id),
                station_id_column.eq(sensor.station_id)
            ))
            .get_result(&conn)?;
        Ok(sensor)
    }

    pub fn update(id: uuid::Uuid, sensor: SensorsChangeset) -> Result<Self, CustomError> {
        use crate::schema::sensors::dsl::{
            alias as alias_column,
            label as label_column,
        };

        let conn = db::connection()?;
        let sensor = diesel::update(sensors::table)
            .filter(sensors::id.eq(id))
            .set((
                alias_column.eq(sensor.alias),
                label_column.eq(sensor.label),
            ))
            .get_result(&conn)?;
        Ok(sensor)
    }

    // Update timestamp
    pub fn touch(id: Uuid) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let sensor: Self = sensors::table.filter(sensors::id.eq(id)).first(&conn)?;
        let sensor = diesel::update(sensors::table)
            .filter(sensors::id.eq(id))
            .set((
                sensors::alias.eq(sensor.alias),
                sensors::label.eq(sensor.label),
                sensors::type_id.eq(sensor.r#type_id),
                sensors::station_id.eq(sensor.station_id),
                sensors::updated_at.eq(Local::now().naive_local())
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
            label: sensor.label,
            type_id: sensor.type_id,
            station_id: sensor.station_id
        }
    }
}
