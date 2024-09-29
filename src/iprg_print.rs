use crate::{
    client::{DnsError, DnsHttpsServer, DnsResponse, PrintHandler, QueryType},
    iprg::searcher::{search_by_ip, searcher_init},
};

pub fn init_iprg(xdb_filepath: Option<String>) {
    // 初始化加载xdb文件
    let mut xdb_filepath = xdb_filepath;
    if xdb_filepath.is_none() {
        let envxdb = std::env::var("XDB_FILEPATH");
        if let Ok(ex) = envxdb {
            xdb_filepath = Some(ex);
        }
    }
    searcher_init(xdb_filepath);
}

#[derive(Default)]
pub struct Ip2RegionPrintHandler;
impl PrintHandler for Ip2RegionPrintHandler {
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
                let qtype = QueryType::from_u16(ans.r#type).unwrap();
                println!("  ------------");
                println!("  Name: {}", ans.name);
                println!("  Type: {}", qtype);
                println!("  TTL: {}", ans.ttl);
                println!("  Data: {}", ans.data);
                if qtype.eq(&QueryType::A) {
                    let result = search_by_ip(ans.data.as_str());
                    if let Ok(result) = result {
                        println!("  IPRG: {}", result);
                    }
                }
            }
        }
        if let Some(comment) = &response.comment {
            println!("Comment: {}", comment);
        }
    }
}
