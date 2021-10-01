#[macro_use]
extern crate diesel_migrations;

use api::homepage;
use api::stations;
use api::sensor_types;
use api::sensors;
use api::udp::{UdpServer, udp_server};
use actix_web::{App, HttpServer, middleware::Logger, middleware::DefaultHeaders};
use dotenv::dotenv;
use log::info;
use listenfd::ListenFd;
use std::{env, net::UdpSocket, thread};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use api::db::Pool;



embed_migrations!();


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let db_url = env::var("DATABASE_URL").expect("Database url not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::new(manager).expect("Failed to create db pool");
    //Running DB migrations
    let conn = pool.get().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
    let thread_pool = pool.clone();
    thread::spawn(move || {
        let address = env::var("UDP").expect("Please set the UDP environment variable in .env");
        let udp_socket = UdpSocket::bind(address.clone()).unwrap();
        info!("Listening on UDP address {}", address);
        udp_server(UdpServer::new(udp_socket),thread_pool).unwrap();
    });


    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move ||
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(DefaultHeaders::new().header("Access-Control-Allow-Methods", "GET"))
            .wrap(DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .configure(stations::init_routes)
            .configure(sensors::init_routes)
            .configure(sensor_types::init_routes)
            .configure(homepage::init_routes)
    );

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let address = env::var("HTTP").expect("Please set the HTTP environment variable in .env");
            server.bind(address)?
        }
    };

    server.run().await
}
