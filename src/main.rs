use clap::Parser;
use colored::Colorize;
use std::net::{SocketAddr, ToSocketAddrs};

mod args;
mod ports;
mod scan;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let scanip_args = args::ScanIp::parse();

    let socket_addresses: Vec<SocketAddr> = format!("{}:0", scanip_args.target)
        .to_socket_addrs()?
        .collect();

    if socket_addresses.is_empty() {
        return Err(anyhow::anyhow!("socket addresses list is empty".red()));
    }

    if scanip_args.verbose {
        let ports = if scanip_args.full {
            "all the 65535 ports".yellow()
        } else {
            "the most common 1002 ports".blue()
        };
        println!(
            "start scanning {} of {};\n  {};\n  {};",
            &ports,
            scanip_args.target.green(),
            format!("set concurrency = {}", scanip_args.concurrency)
                .bold()
                .cyan(),
            format!("set timeout = {}s", scanip_args.timeout)
                .bold()
                .cyan()
        );
        println!("-----");
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
