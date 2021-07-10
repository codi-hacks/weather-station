use std::net::UdpSocket;
use clap::Clap;
use crc::{crc32, Hasher32};

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " (", env!("GIT_HASH"), ")");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Clap)]
#[clap(version = VERSION, author = AUTHORS)]
struct Opts {
    server: String,
    station_id: String,
    station_key: String,
    measurements: String,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let socket = UdpSocket::bind(":::0")?;
    socket.connect(opts.server)?;
    let mut msg = opts.measurements + &format!(",id={}", opts.station_id);
    msg += "#";
    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(msg.as_bytes());
    digest.write(opts.station_key.as_bytes());
    let checksum = digest.sum32();
    msg += &format!("{}", checksum);
    socket.send(msg.as_bytes())?;
    Ok(())
}
