use crate::types::ScanConfig;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use tokio::task;

pub struct TcpScanner {
    config: ScanConfig,
}

impl TcpScanner {
    pub fn new(config: ScanConfig) -> Self {
        Self { config }
    }

    fn get_common_ports() -> Vec<u16> {
        vec![21, 22, 23, 25, 53, 80, 110, 443, 3389, 8080, 8443]
    }

    pub async fn scan(&self, target: &str) -> Vec<u16> {
        let ports = Self::get_common_ports();
        let target = target.to_string();
        let timeout = Duration::from_millis(self.config.timeout_ms);

        task::spawn_blocking(move || {
            ports.into_iter()
                .filter(|port| {
                    let addr: SocketAddr = match format!("{}:{}", target, port).parse() {
                        Ok(addr) => addr,
                        Err(_) => return false,
                    };
                    TcpStream::connect_timeout(&addr, timeout).is_ok()
                })
                .collect()
        }).await.unwrap()
    }
}
