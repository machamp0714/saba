extern crate alloc;
use crate::alloc::string::ToString;
use alloc::format;
use alloc::string::String;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        // ホスト名からIPアドレスを取得
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                    e
                )))
            }
        };

        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP addresses".to_string()));
        }

        // ソケットアドレスの定義
        let socket_addr: SocketAddr = (ips[0], port).into();

        // ストリームの構築
        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network(format!(
                    "Failed to connect to TCP stream".to_string()
                )))
            }
        }

        // リクエストラインの構築
        let mut request = String::from("GET /");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\n");

        // ヘッダの構築
        request.push_str("Host: ");
        request.push_str(&host);
        request.push_str("\n");
        request.push_str("Accept: text/html\n");
        request.push_str("Connection: close\n");
        request.push_str("\n");

        // リクエストの送信
        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to send a request to TCP stream".to_string(),
                ))
            }
        }
    }
}