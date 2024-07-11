use std::collections::BTreeMap;
use std::fmt;
use std::net::IpAddr;
use std::os::unix::io::AsRawFd;

use smoltcp::iface::{EthernetInterfaceBuilder, NeighborCache, Routes};
use smoltcp::phy::{wait as phy_wait, TapInterface};
use smoltcp::socket::{SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};
use url::Url;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    Network(smoltcp::Error),
    InvalidUrl,
    Content(std::str::Utf8Error),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<smoltcp::Error> for UpstreamError {
    fn from(error: smoltcp::Error) -> Self {
        UpstreamError::Network(error)
    }
}

impl From<std::str::Utf8Error> for UpstreamError {
    fn from(error: std::str::Utf8Error) -> Self {
        UpstreamError::Content(error)
    }
}

fn random_port() -> u16 {
    49152 + rand::random::<u16>() % 16384
}

pub fn get(tap:TapInterface, mac: EthernetAddress, addr: IpAddr, url: Url) -> Result<(), UpstreamError> {
    todo!();
}