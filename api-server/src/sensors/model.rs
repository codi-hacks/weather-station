use crate::db;
use crate::error_handler::CustomError;
use crate::schema::{sensors, sensor_types, stations};
use crate::stations::{Station, StationsModel};
use crate::measurements::MeasurementsModel;
use crate::sensor_types::{SensorType, SensorTypesModel};
use chrono::{Duration, Local, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Sensor {
    pub id: Uuid,
    pub alias: String,
    pub label: String,
    pub measurements: Option<Vec<MeasurementsModel>>,
    pub r#type: Option<SensorType>,
    #[serde(skip_serializing)]
    pub type_id: Uuid,
    pub station: Option<Station>,
    #[serde(skip_serializing)]
    pub station_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "sensors"]
pub struct NewSensor {
    pub alias: String,
    pub label: String,
    pub type_id: Uuid,
    pub station_id: Uuid
}

#[derive(AsChangeset, Associations, Clone, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[belongs_to(SensorTypesModel, foreign_key = "type_id")]
#[belongs_to(StationsModel, foreign_key = "station_id")]
#[table_name = "sensors"]
pub struct SensorsModel {
    pub id: Uuid,
    pub alias: String,
    pub label: String,
    // Be consistent and never send *_id fields. If the
    // assocation is needed we'll nest that model's struct.
    #[serde(skip_serializing)]
    pub type_id: Uuid,
    #[serde(skip_serializing)]
    pub station_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl SensorsModel {
    pub fn find_all() -> Result<Vec<Sensor>, CustomError> {
        let conn = db::connection()?;
        let sensors = sensors::table.load::<SensorsModel>(&conn)?;
        let sensor_types = SensorTypesModel::as_hash()?;
        let stations = StationsModel::as_hash()?;

        let sensors = sensors.into_iter().filter_map(|sensor| {
            match sensor_types.get(&sensor.type_id.to_string()) {
                Some(sensor_type) =>
                    match stations.get(&sensor.station_id.to_string()) {
                        Some(station) => Some(Sensor {
                            id: sensor.id,
                            alias: sensor.alias,
                            label: sensor.label,
                            measurements: None,
                            r#type: Some(SensorType {
                                id: sensor_type.id,
                                label: sensor_type.label.clone(),
                                description: sensor_type.description.clone(),
                                sensors: None
                            }),
                            type_id: sensor_type.id,
                            station: Some(Station {
                                id: station.id,
                                label: station.label.clone(),
                                key: station.key.clone(),
                                sensors: None
                            }),
                            station_id: station.id,
                            created_at: sensor.created_at,
                            updated_at: sensor.updated_at
                        }),
                        None => None
                    }
                None => None
            }
        }).collect();

        Ok(sensors)
    }

    pub fn find(id: Uuid) -> Result<Sensor, CustomError> {
        use crate::schema::measurements::dsl::{created_at as measurement_created_at};

        let conn = db::connection()?;
        let time_ago: NaiveDateTime = Local::now().naive_local() - Duration::days(90);
        let sensor: Self = sensors::table.filter(sensors::id.eq(id)).first(&conn)?;
        let measurements: Vec<MeasurementsModel> = MeasurementsModel::belonging_to(&sensor)
            .order(measurement_created_at.asc())
            .filter(measurement_created_at.ge(time_ago))
            .limit(50_000)
            .load(&conn)?;
        let sensor_type: SensorTypesModel = sensor_types::table.filter(sensor_types::id.eq(sensor.type_id)).first(&conn)?;
        let sensor_type_id = sensor_type.id.clone();
        let station: StationsModel = stations::table.filter(stations::id.eq(sensor.station_id)).first(&conn)?;
        Ok(Sensor {
            id: sensor.id,
            alias: sensor.alias,
            label: sensor.label,
            measurements: Some(measurements),
            r#type: Some(SensorType {
                id: sensor_type.id,
                label: sensor_type.label,
                description: sensor_type.description,
                sensors: None
            }),
            type_id: sensor_type_id,
            station: Some(Station {
                id: station.id,
                label: station.label,
                key: station.key,
                sensors: None
            }),
            station_id: station.id,
            created_at: sensor.created_at,
            updated_at: sensor.updated_at
        })
    }

    pub fn find_by_station(station: Station) -> Result<Vec<Sensor>, CustomError> {
        let conn = db::connection()?;
        let sensors: Vec<Self> = SensorsModel::belonging_to(&StationsModel {
            id: station.id.clone(),
            label: station.label.clone(),
            key: station.key.clone()
        }).load(&conn)?;
        let sensors = sensors.into_iter().map(|sensor| {
            Sensor {
                id: sensor.id,
                alias: sensor.alias,
                label: sensor.label,
                measurements: None,
                r#type: None,
                type_id: sensor.type_id,
                station: None,
                station_id: station.id.clone(),
                created_at: sensor.created_at,
                updated_at: sensor.updated_at
            }
        }).collect();
        Ok(sensors)
    }

    pub fn create(sensors: Vec<NewSensor>) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;

        let sensors = diesel::insert_into(sensors::table)
            .values(sensors)
            .get_results(&conn)?;
        Ok(sensors)
    }

    // Update data
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

    pub fn delete(id: Uuid) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(sensors::table.filter(sensors::id.eq(id))).execute(&conn)?;
        Ok(res)
    }

    pub fn delete_by_station(station: Station) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let station = StationsModel {
            id: station.id,
            label: station.label,
            key: station.key
        };
        Ok(diesel::delete(SensorsModel::belonging_to(&station)).execute(&conn)?)
    }
}
