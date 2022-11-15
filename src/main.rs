use clap::Parser;
use rustun::server::{BindingHandler, UdpServer};
use anyhow::Result;

enum STUNType {
    Client,
    Server
}

#[derive(Debug, Parser)]
struct Args {
    stun_type: STUNType,
    #[clap(short, long, default_value_t = 3478)]
    port: u16,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let addr = format!("0.0.0.0:{}", args.port).parse()?;

    let server = fibers_global::execute(UdpServer::start(
        fibers_global::handle(),
        addr,
        BindingHandler
    ))?;
    fibers_global::execute(server)?;
    Ok(())
}