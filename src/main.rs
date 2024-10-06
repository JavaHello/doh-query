use std::time::Duration;

use args::Config;
use clap::Parser;
use client::{DefaultPrintHandler, Dns, DnsHttpsServer, HttpsClient};

mod args;
mod client;
mod iprg;
mod iprg_print;

#[tokio::main]
async fn main() {
    let config = Config::parse();
    let client = HttpsClient::new(Duration::from_millis(config.timeout));
    let mut dns = Dns::from_client(client, config.retries);

    let server = config.server;
    match server.as_str() {
        "google" => dns.add_server(DnsHttpsServer::Google),
        "cloudflare" => dns.add_server(DnsHttpsServer::Cloudflare),
        "quad9" => dns.add_server(DnsHttpsServer::Quad9),
        "all" => {
            dns.add_server(DnsHttpsServer::Google);
            dns.add_server(DnsHttpsServer::Cloudflare);
            dns.add_server(DnsHttpsServer::Quad9);
        }
        s if s.starts_with("http") => {
            dns.add_server(DnsHttpsServer::custom("Custom".to_string(), server));
        }
        _ => {
            println!("Invalid server, using default servers");
            dns.add_server(DnsHttpsServer::Cloudflare);
        }
    }

    match config.fmt.as_str() {
        "ip2region" | "iprg" => {
            iprg_print::init_iprg(config.xdb_filepath);
            dns.resolve::<iprg_print::Ip2RegionPrintHandler>(config.domain, config.r#type)
                .await;
        }
        _ => {
            dns.resolve::<DefaultPrintHandler>(config.domain, config.r#type)
                .await;
        }
    }
}
