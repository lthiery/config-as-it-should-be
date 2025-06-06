#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub gateway: Gateway,
}

#[derive(Debug, Deserialize)]
pub struct Gateway {
    pub ip_address: String,
    pub port: u16,
}
