# DNS over HTTPS Query CLI

简单的 DNS over HTTPS 查询命令行工具

## 使用

```shell
# 查看帮助
doh-query -h
Usage: doh-query [OPTIONS] <DOMAIN>

Arguments:
  <DOMAIN>  The domain to resolve

Options:
  -s, --server <SERVER>                Select the DNS server to use: cloudflare, google, quad9, custom [default: cloudflare]
      --timeout <TIMEOUT>              The timeout in milliseconds [default: 10000]
      --retries <RETRIES>              The number of retries [default: 3]
  -c, --custom-server <CUSTOM_SERVER>  The custom DNS server to use
  -t, --type <TYPE>                    The type of record to resolve [default: A]
  -f, --fmt <FMT>                      output format: default, ip2region [default: default]
      --xdb-filepath <XDB_FILEPATH>    The path to the IP2Region database file
  -h, --help                           Print help
  -V, --version                        Print version

# 查询域名
doh-query google.com
---------------------------------
Dns Server: Cloudflare
Status: 0
Question:
  ------------
  Name: google.com
  Type: A
Answer:
  ------------
  Name: google.com
  Type: A
  TTL: 270
  Data: 142.250.71.142


```

## 特性

- 内置 google, cloudflare, quad9 三个 DNS over HTTPS 服务器
- 支持自定义 DNS over HTTPS 服务器
- 支持 `ip2region` 查询 IP 所在地, 依赖 `ip2region` 数据库文件
