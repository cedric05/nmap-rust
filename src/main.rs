use anyhow::Result;
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
    /// start port
    #[clap(short, long, default_value_t = 1)]
    start: u16,

    /// end port
    #[clap(short, long, default_value_t = 1024)]
    end: u16,

    /// subnet to look into
    #[clap(short, long)]
    ip: String,

    /// connection timeout
    #[clap(short, long, default_value_t = 10)]
    timeout: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let ipnet = IpNet::from(Ipv4Net::from_str(&args.ip)?);
    ipnet
        .subnets(ipnet.max_prefix_len())?
        .flat_map(|x| (args.start..=args.end).map(move |port| (x, port)))
        .par_bridge()
        .for_each(|(ip, port)| {
            if TcpStream::connect_timeout(
                &SocketAddr::new(ip.addr(), port),
                Duration::from_millis(args.timeout),
            )
            .is_ok()
            {
                println!("success ip={:?} port={x}", ip.addr(), x = port);
            }
        });
    Ok(())
}
