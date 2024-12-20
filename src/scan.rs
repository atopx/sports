use futures::{stream, StreamExt};
use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};
use tokio::net::TcpStream;

pub async fn scanner(
    target: IpAddr,
    ports: Box<dyn Iterator<Item = u16>>,
    concurrency: usize,
    timeout: u64,
) {
    let ports = stream::iter(ports);

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target, port);

    if let Ok(Ok(_)) = tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        println!("{}", port)
    }
}
