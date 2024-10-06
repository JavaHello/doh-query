use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    ///Select the DNS server to use: cloudflare, google, quad9, custom
    #[clap(short, long, default_value = "cloudflare")]
    pub server: String,

    /// The timeout in milliseconds
    #[clap(long, default_value = "10000")]
    pub timeout: u64,

    /// The number of retries
    #[clap(long, default_value = "3")]
    pub retries: u8,

    /// The type of record to resolve
    #[clap(short, long, default_value = "A")]
    pub r#type: String,

    /// output format: default, ip2region
    #[clap(short, long, default_value = "default")]
    pub fmt: String,

    /// The path to the IP2Region database file
    #[clap(long)]
    pub xdb_filepath: Option<String>,

    /// The domain to resolve
    pub domain: String,
}
