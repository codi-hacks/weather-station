use crate::db;
use crate::error_handler::CustomError;
use crate::schema::sensors;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::sensor_types::{SensorTypesModel};
use crate::stations::{StationsModel};

//New import
use crate::measurements::MeasurementsModel;


#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "sensors"]
pub struct SensorsChangeset {
    pub alias: String,
    pub label: String,
    pub type_id: uuid::Uuid,
    pub station_id: uuid::Uuid
}

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


//#[derive(Serialize, Deserialize, Associations, Queryable, Insertable, Identifiable)]
#[derive(AsChangeset, Associations, Clone, Deserialize, Identifiable, Insertable, Queryable, Serialize)] 
#[belongs_to(SensorTypesModel, foreign_key = "type_id")]
#[belongs_to(StationsModel, foreign_key = "station_id")]
#[table_name = "sensors"]
pub struct SensorsModel {
    pub id: uuid::Uuid,
    pub alias: String,
    pub label: String,
    pub type_id: uuid::Uuid,
    pub station_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}

impl SensorsModel {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let sensors = sensors::table.load::<Self>(&conn)?;
        Ok(sensors)
    }

    pub fn find(id: uuid::Uuid) -> Result<Sensor, CustomError> {
        let conn = db::connection()?;
        let sensor: SensorsModel = sensors::table.filter(sensors::id.eq(id)).first(&conn)?;
        //Ok(sensor)    //Original Ok() line
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
