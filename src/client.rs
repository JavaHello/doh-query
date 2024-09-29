use std::{fmt::Display, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum QueryType {
    A,
    Aaaa,
    Caa,
    Cname,
    MX,
    NS,
    Ptr,
    Soa,
    Srv,
    Txt,
    Unknown(u16),
}
impl Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryType::A => write!(f, "A"),
            QueryType::Aaaa => write!(f, "AAAA"),
            QueryType::Caa => write!(f, "CAA"),
            QueryType::Cname => write!(f, "CNAME"),
            QueryType::MX => write!(f, "MX"),
            QueryType::NS => write!(f, "NS"),
            QueryType::Ptr => write!(f, "PTR"),
            QueryType::Soa => write!(f, "SOA"),
            QueryType::Srv => write!(f, "SRV"),
            QueryType::Txt => write!(f, "TXT"),
            QueryType::Unknown(u) => write!(f, "Unknown({})", u),
        }
    }
}

impl QueryType {
    pub fn from_u16(u: u16) -> Option<Self> {
        match u {
            1 => Some(QueryType::A),
            28 => Some(QueryType::Aaaa),
            257 => Some(QueryType::Caa),
            5 => Some(QueryType::Cname),
            15 => Some(QueryType::MX),
            2 => Some(QueryType::NS),
            12 => Some(QueryType::Ptr),
            6 => Some(QueryType::Soa),
            33 => Some(QueryType::Srv),
            16 => Some(QueryType::Txt),
            _ => Some(QueryType::Unknown(u)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DnsHttpsConfig {
    name: String,
    url: String,
}
#[derive(Debug, Clone)]
pub enum DnsHttpsServer {
    Google,
    Cloudflare,
    Quad9,
    Custom(DnsHttpsConfig),
}

impl DnsHttpsServer {
    pub fn custom(name: String, url: String) -> Self {
        DnsHttpsServer::Custom(DnsHttpsConfig { name, url })
    }

    pub fn url(&self) -> String {
        match self {
            DnsHttpsServer::Google => "https://dns.google/resolve".to_string(),
            DnsHttpsServer::Cloudflare => "https://cloudflare-dns.com/dns-query".to_string(),
            DnsHttpsServer::Quad9 => "https://9.9.9.9:5053/dns-query".to_string(),
            DnsHttpsServer::Custom(config) => config.url.to_string(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            DnsHttpsServer::Google => "Google".to_string(),
            DnsHttpsServer::Cloudflare => "Cloudflare".to_string(),
            DnsHttpsServer::Quad9 => "Quad9".to_string(),
            DnsHttpsServer::Custom(config) => config.name.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQuestion {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsAnswer {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: u16,
    #[serde(rename = "TTL")]
    pub ttl: u32,
    #[serde(rename = "expires")]
    pub expires: Option<String>,
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsAuthority {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub r#type: u16,

    #[serde(rename = "TTL")]
    pub ttl: u32,

    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsResponse {
    #[serde(rename = "error")]
    pub error: Option<String>,

    #[serde(rename = "Status")]
    pub status: Option<u16>,

    #[serde(rename = "TC")]
    tc: Option<bool>,

    #[serde(rename = "RD")]
    rd: Option<bool>,

    #[serde(rename = "RA")]
    ra: Option<bool>,

    #[serde(rename = "AD")]
    ad: Option<bool>,

    #[serde(rename = "CD")]
    cd: Option<bool>,

    #[serde(rename = "Question")]
    pub question: Option<Vec<DnsQuestion>>,

    #[serde(rename = "Answer")]
    pub answer: Option<Vec<DnsAnswer>>,

    #[serde(rename = "Authority")]
    pub authority: Option<Vec<DnsAuthority>>,

    #[serde(rename = "Comment")]
    pub comment: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsError {
    pub code: u16,
    pub message: String,
}

pub trait DnsClient {
    async fn query(&self, uri: String) -> Result<DnsResponse, DnsError>;
}

pub trait PrintHandler {
    fn handle(server: &DnsHttpsServer, result: &Result<DnsResponse, DnsError>);
}

#[derive(Default)]
pub struct DefaultPrintHandler;
impl PrintHandler for DefaultPrintHandler {
    fn handle(server: &DnsHttpsServer, result: &Result<DnsResponse, DnsError>) {
        println!("---------------------------------");
        println!("Dns Server: {}", server.name());
        if let Err(err) = result {
            println!("Error: {}", err.message);
            return;
        }
        let response = result.as_ref().unwrap();

        if let Some(status) = response.status {
            println!("Status: {}", status);
        }

        if let Some(error) = &response.error {
            println!("Error: {}", error);
        }
        if let Some(question) = &response.question {
            println!("Question:");
            for ques in question {
                println!("  ------------");
                println!("  Name: {}", ques.name);
                println!("  Type: {}", QueryType::from_u16(ques.r#type).unwrap());
            }
        }

        if let Some(authority) = &response.authority {
            println!("Authority:");
            for auth in authority {
                println!("  ------------");
                println!("  Name: {}", auth.name);
                println!("  Type: {}", QueryType::from_u16(auth.r#type).unwrap());
                println!("  TTL: {}", auth.ttl);
                println!("  Data: {}", auth.data);
            }
        }
        if let Some(answer) = &response.answer {
            println!("Answer:");
            for ans in answer {
                println!("  ------------");
                println!("  Name: {}", ans.name);
                println!("  Type: {}", QueryType::from_u16(ans.r#type).unwrap());
                println!("  TTL: {}", ans.ttl);
                println!("  Data: {}", ans.data);
            }
        }
        if let Some(comment) = &response.comment {
            println!("Comment: {}", comment);
        }
    }
}

pub struct Dns<C: DnsClient> {
    client: C,
    servers: Vec<DnsHttpsServer>,
    pub retries: u8,
}

impl<C: DnsClient> Dns<C> {
    pub fn from_client(client: C, retries: u8) -> Self {
        Dns {
            client,
            servers: Vec::new(),
            retries,
        }
    }

    pub fn add_server(&mut self, server: DnsHttpsServer) {
        self.servers.push(server);
    }

    pub async fn resolve<T: PrintHandler>(&self, name: String, r#type: String) {
        for server in &self.servers {
            let uri = server.url();

            for _ in 0..self.retries {
                let result = self
                    .client
                    .query(format!("{}?name={}&type={}", uri, name, r#type))
                    .await;

                T::handle(server, &result);
                if result.is_ok() {
                    break;
                }
            }
        }
    }
}

pub struct HttpsClient {
    pub timeout: Duration,
}

impl HttpsClient {
    pub fn new(timeout: Duration) -> Self {
        HttpsClient { timeout }
    }
}

impl DnsClient for HttpsClient {
    async fn query(&self, uri: String) -> Result<DnsResponse, DnsError> {
        let send = reqwest::Client::new()
            .get(uri)
            .timeout(self.timeout)
            .header("accept", "application/dns-json")
            .send()
            .await;
        if let Err(err) = send {
            return Err(DnsError {
                code: 500,
                message: format!("{:?}", err),
            });
        }
        let send = send.unwrap();
        match send.status() {
            reqwest::StatusCode::OK => {}
            reqwest::StatusCode::BAD_REQUEST => {
                return Err(DnsError {
                    code: 400,
                    message: "DNS query not specified or too small.".to_string(),
                });
            }
            reqwest::StatusCode::PAYLOAD_TOO_LARGE => {
                return Err(DnsError {
                    code: 413,
                    message: "DNS query is larger than maximum allowed DNS message size."
                        .to_string(),
                });
            }
            _ => {
                return Err(DnsError {
                    code: send.status().as_u16(),
                    message: format!("{:?}", send.text().await),
                });
            }
        }
        let response = send.text().await;
        if let Err(err) = response {
            return Err(DnsError {
                code: 500,
                message: format!("{:?}", err),
            });
        }
        let resp_body = response.unwrap();
        serde_json::from_str::<DnsResponse>(&resp_body).map_err(|err| DnsError {
            code: 500,
            message: format!("{:?}, body: {}", err, resp_body),
        })
    }
}

mod tests {

    #[tokio::test]
    async fn test_dns() {
        use super::{DefaultPrintHandler, Dns, DnsHttpsServer, HttpsClient};
        use std::time::Duration;
        let client = HttpsClient::new(Duration::from_secs(5));
        let mut dns = Dns::from_client(client, 1);
        dns.add_server(DnsHttpsServer::Google);
        dns.add_server(DnsHttpsServer::Cloudflare);
        dns.resolve::<DefaultPrintHandler>("example.com".to_string(), "A".to_string())
            .await;
    }
}
