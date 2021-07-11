use std::{
    net::UdpSocket,
    collections::HashMap,
};
use crate::error_handler::CustomError;
use crate::measurements::MeasurementsModel;
use crate::stations::StationsModel;
use bigdecimal::BigDecimal;
use crc::{crc32, Hasher32};
use log::{trace, debug, warn, info};
use nom;
use nom::sequence::separated_pair;
use nom::bytes::complete::tag;
use nom::number::complete::double;
use nom::bytes::complete::take_till;
use nom::combinator::opt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum UdpError {
    Custom(String),
}

impl<I> From<nom::Err<(I, nom::error::ErrorKind)>> for UdpError {
    fn from(_: nom::Err<(I, nom::error::ErrorKind)>) -> Self {
        UdpError::Custom("Problem parsing UDP message".into())
    }
}

pub struct Message {
    pub measurements: HashMap<String, BigDecimal>,
    pub station_id: String,
    // The digest so far; notably missing the station key
    pub digest: crc32::Digest,
    // The expected checksum
    pub checksum: usize,
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
         .field("measurements", &self.measurements)
         .field("station_id", &self.station_id)
         .field("checksum", &self.checksum)
         .finish()
    }
}

/// runs an infinite loop to process UDP messages
pub fn udp_server(server: UdpServer) -> std::result::Result<(), CustomError> {
    for msg in server {
        // Validate the station ID
        info!("UDP message {:?}", msg);
        let station_id = match Uuid::parse_str(&msg.station_id) {
            Ok(id) => id,
            Err(e) => {
                info!("Received an invalid station ID ({:?})", e);
                continue
            }
        };
        let station = match StationsModel::find(station_id) {
            Ok(s) => s,
            Err(e) => {
                info!("Could not find specified station {:?}", e);
                continue
            }
        };
        debug!("Found station {} - {}", station.id, station.label);
        // Validate the message CRC
        let mut digest = msg.digest;
        digest.write(station.key.as_bytes());
        let our_checksum = digest.sum32() as usize;
        if our_checksum != msg.checksum {
            info!("Dropping message with invalid checksum (got {}, want {})", our_checksum, msg.checksum);
            continue
        } else {
            debug!("Checksums match! {}", our_checksum);
        }
        // Look up sensors by their alias
        let mut sensors_map = HashMap::new();
        for sensor in station.sensors {
            sensors_map.insert(sensor.alias.to_string(), sensor);
        }
        for (alias, measurement_value) in msg.measurements {
            let sensor_id = match sensors_map.get(&alias) {
                Some(s) => s.id,
                None => {
                    info!("Failed to find sensor with alias \"{}\". Skipping.", alias);
                    continue
                }
            };
            match MeasurementsModel::create(sensor_id, measurement_value.clone()) {
                Ok(m) => debug!("Successfully written measurement: {}={}", alias, m.value),
                Err(error) => warn!("{}", error)
            };
        }
    }
    Ok(())
}


/// Parses a slice with the form "key=value", seperated by commas. Expects only
/// valid fields; pressure, air_temp, voltage, signal, id.
fn parse_message(mut input: &[u8]) -> Result<(&[u8], Message), UdpError> {
    let mut values: HashMap<String, BigDecimal> = HashMap::new();
    // A reference to the start of input
    let full_input = input;
    let full_length = input.len();
    let mut station_id = String::new();
    while input.len() != 0 {
        // Check to see if we hit the 'end of message' tag
        let (rest, val) = opt(tag("#"))(input)?;
        if let Some(_) = val {
            input = rest;
            break;
        }
        trace!("parsing next measure; remaining: {}", String::from_utf8_lossy(input));
        let (rest, (name, value)) = separated_pair(
            take_till(|b| b == b'=' ),
            tag("="),
            take_till(|b| b == b',' || b == b'#')
        )(input)?;
        let (rest, _) = opt(tag(","))(rest)?;
        input = rest;
        if name == b"id" {
            station_id = String::from_utf8_lossy(&value).to_string();
        } else {
            match BigDecimal::parse_bytes(value, 10) {
                Some(value) => {
                    values.insert(String::from_utf8_lossy(name).to_string(), value)
                },
                None => {
                    return Err(UdpError::Custom("Received an invalid measurement value".to_string()));
                }
            };
        }
    }
    // Remaining message is crc sum; calculate based on what we've so far
    let message_length = full_length - input.len();
    let digest = perform_crc_check(&full_input[..message_length]);
    let (rest, checksum) = double(input)?;
    let checksum = checksum as usize;
    Ok((rest, Message {
        measurements: values,
        checksum,
        digest,
        station_id,
    }))
}

fn perform_crc_check(input: &[u8]) -> crc32::Digest {
    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(input);
    digest
}

pub struct UdpServer {
    buff: Vec<u8>,
    sock: UdpSocket,
}

impl UdpServer {
    /// constructs a new server which will run on the given address
    pub fn new(sock: UdpSocket) -> Self {
        UdpServer {
            // Max udp packet length
            buff: vec![0; 65535],
            sock,
        }
    }
}

impl Iterator for UdpServer {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let read_result = self.sock.recv(&mut self.buff);
            trace!("read message {:?}", read_result);
            if read_result.is_err() {
                warn!("error reading data from socket; {:?}", read_result);
                return None;
            }
            let count = read_result.unwrap();
            let slice = &self.buff[0..count];
            let msg = parse_message(slice);
            if msg.is_err() {
                warn!("dropping invalid message; error was {:?}", msg);
                continue;
            }
            let msg = msg.unwrap();
            let msg = msg.1;
            return Some(msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    const VALID_MESSAGE: &[u8] = b"pressure=987.89,air_temp=25.46,voltage=3.93,signal=-42,id=hello-world#12345";

    fn create_udp_server() -> Result<(UdpSocket, UdpServer), Box<dyn std::error::Error>> {
        let server_socket = UdpSocket::bind(":::0")?;
        let client_socket = UdpSocket::bind(":::0")?;
        client_socket.connect(server_socket.local_addr()?)?;
        Ok((client_socket, UdpServer::new(server_socket)))
    }

    #[test]
    fn test_iterate_some_messages() {
        let (client, server) = create_udp_server().unwrap();
        thread::spawn(move || {
            client.send(VALID_MESSAGE).unwrap();
        });
        let mut count = 0;
        for _ in server {
            count += 1;
            break;
        }
        assert_eq!(1, count);
    }

    #[test]
    fn test_parse_message() {
        let (rest, _) = parse_message(VALID_MESSAGE).unwrap();
        assert_eq!(rest.len(), 0);
        // try a weirdly formed message
        let res = parse_message(b"hello my name is bob");
        assert_eq!(true, res.is_err());
    }
}
