use clap::Parser;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use tokio::task;

/// Simple port scanner
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Target host to scan
    target: String,
    
    /// Timeout in milliseconds
    #[arg(short, long, default_value = "1000")]
    timeout: u64,
}

fn get_common_ports() -> Vec<u16> {
    vec![
        21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995,
        1723, 3306, 3389, 5900, 8080, 8443,
    ]
}

async fn scan_ports(target: &str, timeout: u64) -> Vec<u16> {
    let ports = get_common_ports();
    let target = target.to_string();
    let timeout_duration = Duration::from_millis(timeout);

    task::spawn_blocking(move || {
        ports
            .into_iter()
            .filter(|port| {
                let addr: SocketAddr = match format!("{}:{}", target, port).parse() {
                    Ok(addr) => addr,
                    Err(_) => return false,
                };
                
                TcpStream::connect_timeout(&addr, timeout_duration).is_ok()
            })
            .collect()
    })
    .await
    .unwrap()
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("Scanning common ports on {}...", args.target);
    
    let open_ports = scan_ports(&args.target, args.timeout).await;
    
    if open_ports.is_empty() {
        println!("No open ports found");
    } else {
        println!("Open ports:");
        for port in open_ports {
            println!("  {}", port);
        }
    }
}
