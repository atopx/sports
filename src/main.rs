use clap::Parser;
use colored::Colorize;
use std::net::{SocketAddr, ToSocketAddrs};

mod args;
mod ports;
mod scan;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let scanip_args = args::ScanIp::parse();

    if scanip_args.verbose {
        let ports = if scanip_args.full {
            "all the 65535 ports".yellow()
        } else {
            "the most common 1002 ports".blue()
        };
        println!(
            "Scanning {} of {}. Concurrency: {:?}. Timeout: {:?}s",
            &ports,
            scanip_args.target.green(),
            scanip_args.concurrency,
            scanip_args.timeout
        );
    }

    let socket_addresses: Vec<SocketAddr> = format!("{}:0", scanip_args.target)
        .to_socket_addrs()?
        .collect();

    if socket_addresses.is_empty() {
        return Err(anyhow::anyhow!("Socket_addresses list is empty".red()));
    }

    let ports = ports::get_ports(scanip_args.full);

    scan::scanner(
        socket_addresses[0].ip(),
        ports,
        scanip_args.concurrency,
        scanip_args.timeout,
    )
    .await;

    Ok(())
}
