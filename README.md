# rmap 
Simple port scanner written in Rust for educational purposes.
Features
- TCP Connect scanning
- Scans common ports (21, 22, 23, 25, 53, 80, 110, 443, etc.)
- Configurable timeout
- Async execution

## Build
```Bash
git clone https://github.com/n3tw4lk3r/rmap
cd rmap
cargo build --release
```

## Usage 
```Bash
# Scan common ports on localhost
./target/release/rmap 127.0.0.1

# Scan with custom timeout
./target/release/rmap example.com -t 500

# For development
cargo run -- 127.0.0.1
```

## TODO List:
- Add port range specification (-p 80,443,1000-2000)

- Multi-threading for faster scans

- Service version detection

- SYN scan support (raw sockets)

- Output formatting (JSON, XML)

- Host discovery (ping sweep)

- OS fingerprinting

- Progress bar for long scans

- Config file support

- IPv6 support

## Legal Notice

This tool is for educational purposes only. Only scan networks and systems you own or have explicit permission to test.
