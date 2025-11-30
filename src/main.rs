mod cli;
mod scanner;
mod types;

use cli::Args;
use scanner::TcpScanner;
use types::ScanConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse_args();
    
    println!("Scanning common ports on {}...", args.target);
    
    let config = ScanConfig {
        timeout_ms: args.timeout,
    };
    let scanner = TcpScanner::new(config);
    
    let open_ports = scanner.scan(&args.target).await;
    
    if open_ports.is_empty() {
        println!("No open ports found.");
    } else {
        println!("Open ports:");
        for port in open_ports {
            println!("  {}", port);
        }
    }
    
    Ok(())
}
