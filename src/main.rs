use clap::Parser;
use ipnet::{IpNet, Ipv4Net};
use rayon::prelude::*;

use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

/// Simple program to find exposed ips and ports
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// max port to check
    #[clap(short, long)]
    port: u16,

    /// subnet to look into
    #[clap(short, long)]
    ip: String,
}

fn main() {
    let args = Args::parse();
    let net4 = IpNet::from(Ipv4Net::from_str(&args.ip).unwrap());
    let port = args.port;
    let t = net4.subnets(32).unwrap();
    t.par_bridge().for_each(|ip| {
        (1..port + 1).for_each(|x| {
            if TcpStream::connect_timeout(&SocketAddr::new(ip.addr(), x), Duration::from_millis(10))
                .is_ok()
            {
                println!("ip={:?} port={x}", ip.addr(), x = x);
            }
        })
    });
}
