use api::measurements::MeasurementsModel;
use api::stations::{Station, StationsModel};
use api::sensor_types::SensorTypesModel;
use api::sensors::{NewSensor, SensorsModel};
use clap::Clap;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use dotenv::dotenv;
use std::process;
use uuid::Uuid;

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " (", env!("GIT_HASH"), ")");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// Manage weather stations and their sensors using the database URL in the .env file
#[derive(Clap)]
#[clap(version = VERSION, author = AUTHORS)]

struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap)]
enum SubCommand {
    Create(Create),
    Delete(Delete),
    Clean(Clean)
}

/// Register a new station and sensors
#[derive(Clap)]
struct Create {
    /// The display name for this weather station
    #[clap(short, long)]
    label: Option<String>,
    #[clap(short, long)]
    sensors: Option<String>
}

/// Delete an existing station and all its sensors
#[derive(Clap)]
struct Delete {
    /// The UUID of the station you would like to delete
    id: Option<String>
}

/// Delete all measurements on a station but keep the station and its sensors
#[derive(Clap)]
struct Clean {
    /// The UUID of the station you would like to clean out
    id: Option<String>
}

fn main() {
    let opts: Opts = Opts::parse();
    dotenv().ok();

    match opts.subcmd {
        SubCommand::Create(subopts) => create_routine(subopts),
        SubCommand::Delete(subopts) => delete_routine(subopts),
        SubCommand::Clean(subopts) => clean_routine(subopts)
    }
}

fn create_routine(opts: Create) {
    let default_sensors: String = "air_temp,humidity,pressure,altitude,signal,voltage".to_string();
    let default_sensor_labels = vec![
        "Air temperature",
        "Humidity",
        "Pressure",
        "Altitude",
        "Signal strength",
        "Voltage"
    ];

    let sensor_types = match SensorTypesModel::find_all() {
        Ok(s) => s,
        Err(_) => {
            println!("Error connecting to db. Is your DATABASE_URL correct?");
            process::exit(1);
        }
    };

    let mut label: String = match opts.label {
        Some(l) => l,
        None => {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("What label would you like for this weather station?")
                .interact_text()
                .unwrap()
        }
    };
    // Remove any trailing whitespace
    label = label.trim_end().to_string();

    let sensor_aliases_string: String = match opts.sensors {
        Some(s) => s,
        None => {
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Type the aliases of your sensors, separated by commas, or press enter to accept the defaults:\n")
                .default(default_sensors.clone())
                .interact_text()
                .unwrap()
        }
    };
    let sensor_aliases: Vec<&str> = sensor_aliases_string.split(',').collect();

    //
    // Type label for each sensor (or apply defaults)
    //
    let mut sensor_labels = vec![];
    if sensor_aliases_string == default_sensors {
        sensor_labels = (0..sensor_aliases.len()).map(|i| default_sensor_labels[i].to_owned()).collect();
    } else {
        for alias in sensor_aliases.clone() {
            sensor_labels.push(
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Type a user-friendly label for \"{}\" e.g. \"Air temperature\"", alias))
                    .interact_text()
                    .unwrap()
            )
        }
    }

    //
    // Select type for each sensor (or apply defaults)
    //

    let mut sensor_type_ids = vec![];
    if sensor_aliases_string == default_sensors {
        sensor_type_ids = (0..sensor_aliases.len()).map(|i| sensor_types[i].id).collect();
    } else {
        let choices = sensor_types.iter()
            .map(|s| format!("{} - {}", s.label, s.description))
            .collect::<Vec<_>>();
        // Which option is highlighted (we'll persist the position across prompts)
        let mut cursor_position = 0;
        for s in sensor_aliases.clone() {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("What type of sensor is \"{}\"?", s))
                .default(cursor_position)
                .items(&choices[..])
                .interact()
                .unwrap();
            sensor_type_ids.push(sensor_types[selection].id);
            // Update default cursor position
            cursor_position = selection;
        }
    }

    //
    // Write everything to the database
    //

    let station = match StationsModel::create(label) {
        Ok(s) => s,
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    };

    let sensors = sensor_aliases.into_iter()
        .zip(sensor_labels)
        .zip(sensor_type_ids)
        .map(|((alias, label), type_id)| {
            NewSensor {
                alias: alias.to_string(),
                label: label.to_string(),
                type_id,
                station_id: station.id
            }
        })
        .collect();

    SensorsModel::create(sensors).unwrap();

    println!("\nStation \"{}\" created. Use the following ID and key to write measurements via the UDP socket:", station.label);
    println!("ID: {}", station.id);
    println!("Key: {}", station.key);
}

fn delete_routine(opts: Delete) {
    let station: Station = match opts.id {
        Some(id) => {
            match StationsModel::find(Uuid::parse_str(&id[..]).unwrap()) {
                Ok(s) => s,
                Err(_) => {
                    println!("Error. No station matching ID \"{}\"", id);
                    process::exit(1);
                }
            }
        },
        None => {
            let stations = match StationsModel::find_all() {
                Ok(s) => s,
                Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                }
            };
            let choices = stations.iter()
                .map(|s| format!("{} - {}", s.id, s.label))
                .collect::<Vec<_>>();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which station to delete?")
                .default(choices.len() - 1)
                .items(&choices[..])
                .interact()
                .unwrap();

            stations[selection].clone()
        }
    };

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Are you sure you want to delete {} ({}) and ALL its sensor data?", station.id, station.label))
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        let mut measurements_deleted: usize = 0;
        for sensor in SensorsModel::find_by_station(station.clone()).unwrap() {
            measurements_deleted += MeasurementsModel::delete_by_sensor(sensor).unwrap();
        }
        let sensors_deleted = SensorsModel::delete_by_station(station.clone()).unwrap();
        StationsModel::delete(station.id).unwrap();
        println!("OK. {} sensors deleted. {} measurements deleted.", sensors_deleted, measurements_deleted);
    } else {
        println!("Action aborted");
        process::exit(1);
    }
}

fn clean_routine(opts: Clean) {
    let station: Station = match opts.id {
        Some(id) => {
            match StationsModel::find(Uuid::parse_str(&id[..]).unwrap()) {
                Ok(s) => s,
                Err(_) => {
                    println!("Error. No station matching ID \"{}\"", id);
                    process::exit(1);
                }
            }
        },
        None => {
            let stations = match StationsModel::find_all() {
                Ok(s) => s,
                Err(error) => {
                    println!("{}", error);
                    process::exit(1);
                }
            };
            let choices = stations.iter()
                .map(|s| format!("{} - {}", s.id, s.label))
                .collect::<Vec<_>>();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which station to clean?")
                .default(choices.len() - 1)
                .items(&choices[..])
                .interact()
                .unwrap();

            stations[selection].clone()
        }
    };

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Are you sure you want to wipe all measurements for {} ({})? Note this will preserve the station and sensors.", station.id, station.label))
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        let mut measurements_deleted: usize = 0;
        for sensor in SensorsModel::find_by_station(station.clone()).unwrap() {
            measurements_deleted += MeasurementsModel::delete_by_sensor(sensor).unwrap();
        }
        println!("OK. {} measurements deleted.", measurements_deleted);
    } else {
        println!("Action aborted");
        process::exit(1);
    }
}
