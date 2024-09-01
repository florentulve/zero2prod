use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug)]
pub struct Application {
    name: String,
    version: String,
    description: String,
    ip: Ipv4Addr,
    port: u16,
}

impl Application {
    pub fn new() -> Self {
        Application {
            ..Default::default()
        }
    }

    pub fn ip<T: Into<Ipv4Addr>>(mut self, ip: T) -> Self {
        self.ip = ip.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn bind(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(self.ip), self.port)
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            name: "zero2prod".to_string(),
            version: "0.1.0".to_string(),
            description: "A simple web application".to_string(),
            ip: "127.0.0.1".parse().unwrap(),
            port: 8080,
        }
    }
}
