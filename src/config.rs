use log::{debug, info};
use num_cpus;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

pub fn get_server_address() -> SocketAddr {
    let default_address = IpAddr::from_str("127.0.0.1").unwrap();
    let default_port = 8088;

    let mut address: IpAddr = default_address;
    let mut port: u16 = default_port;

    match env::var("BIND_ADDRESS") {
        Ok(val) => {
            address = IpAddr::from_str(&val).unwrap();
            info!("Setting address to {:?}", address);
        }
        Err(_e_) => info!(
            "No BIND_ADDRESS environment variable set. Using default {}",
            address
        ),
    }

    match env::var("BIND_PORT") {
        Ok(val) => {
            port = u16::from_str(&val).unwrap();
            info!("Setting port to {}", port);
        }
        Err(_e) => info!(
            "No BIND_PORT environment variable set. Using default {}",
            port
        ),
    };
    debug!("Socket address: {:?} port {}", address, port);
    SocketAddr::new(address, port)
}

pub fn get_worker_count() -> usize {
    let mut workers = num_cpus::get();
    match env::var("WORKERS") {
        Ok(val) => {
            workers = usize::from_str(&val).unwrap();
            info!("Setting worker count to {}", workers);
        }
        Err(_e) => info!("No WORKERS environment variable. Using default {}", workers),
    };
    workers
}
