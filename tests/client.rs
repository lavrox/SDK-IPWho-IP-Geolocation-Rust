//! Runs against a local stand-in server that replays recorded API responses.
//! No API key and no network access are needed.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use ipwho_sdk::{IPWhoClient, IpWhoError};
use reqwest::{StatusCode, Url};

const KEY: &str = "sk.test_secret_value";
const LOOKUP: &str = include_str!("fixtures/lookup.json");
const NULL_CURRENCY: &str = include_str!("fixtures/lookup-null-currency.json");
const BULK: &str = include_str!("fixtures/bulk.json");

#[derive(Debug)]
struct Recorded {
    target: String,
    authorization: Option<String>,
}

struct Stub {
    url: Url,
    reply: Arc<Mutex<(u16, String)>>,
    requests: Arc<Mutex<Vec<Recorded>>>,
}

impl Stub {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = Url::parse(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let reply = Arc::new(Mutex::new((200, LOOKUP.to_owned())));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let (thread_reply, thread_requests) = (reply.clone(), requests.clone());
        thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                let target = line.split_whitespace().nth(1).unwrap_or("").to_owned();
                let mut authorization = None;
                loop {
                    let mut header = String::new();
                    reader.read_line(&mut header).unwrap();
                    if header.trim().is_empty() {
                        break;
                    }
                    if let Some((name, value)) = header.split_once(':') {
                        if name.eq_ignore_ascii_case("authorization") {
                            authorization = Some(value.trim().to_owned());
                        }
                    }
                }
                thread_requests.lock().unwrap().push(Recorded {
                    target,
                    authorization,
                });
                let (status, body) = thread_reply.lock().unwrap().clone();
                write!(
                    stream,
                    "HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\n\
                     Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .unwrap();
            }
        });
        Self {
            url,
            reply,
            requests,
        }
    }

    fn reply(&self, status: u16, body: &str) {
        *self.reply.lock().unwrap() = (status, body.to_owned());
    }

    fn client(&self, key: &str) -> IPWhoClient {
        let http = reqwest::Client::builder().no_proxy().build().unwrap();
        IPWhoClient::with_client(key, http, self.url.clone())
    }

    fn targets(&self) -> Vec<String> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .map(|r| r.target.clone())
            .collect()
    }
}

#[tokio::test]
async fn key_is_in_the_header_and_never_in_the_url() {
    let stub = Stub::start();
    let client = stub.client(KEY);
    client.lookup("8.8.8.8", None, None).await.unwrap();
    client.me(None, None).await.unwrap();
    stub.reply(200, BULK);
    client.bulk(&["8.8.8.8", "1.1.1.1"]).await.unwrap();

    let requests = stub.requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    for request in requests.iter() {
        assert_eq!(request.authorization.as_deref(), Some("Bearer sk.test_secret_value"));
        assert!(!request.target.contains(KEY), "{}", request.target);
        assert!(!request.target.contains('?'), "{}", request.target);
    }
}

#[tokio::test]
async fn lookup_maps_the_response() {
    let stub = Stub::start();
    let data = stub
        .client(KEY)
        .lookup("8.8.8.8", None, None)
        .await
        .unwrap()
        .data
        .unwrap();
    assert_eq!(stub.targets(), ["/ip/8.8.8.8"]);
    assert_eq!(data.ip, "8.8.8.8");
    let geo = data.geo_location.unwrap();
    assert_eq!(geo.country.as_deref(), Some("United States"));
    assert_eq!(geo.country_code.as_deref(), Some("US"));
    assert_eq!(geo.dial_code.as_deref(), Some("+1"));
    assert_eq!(data.connection.unwrap().asn_number, Some(15169));
    assert_eq!(data.currency.unwrap().code.as_deref(), Some("USD"));
}

#[tokio::test]
async fn lookup_accepts_a_null_currency() {
    let stub = Stub::start();
    stub.reply(200, NULL_CURRENCY);
    let data = stub
        .client(KEY)
        .lookup("1.1.1.1", None, None)
        .await
        .unwrap()
        .data
        .unwrap();
    assert_eq!(data.ip, "1.1.1.1");
    assert!(data.currency.is_none());
}

#[tokio::test]
async fn bulk_returns_the_data_for_every_address() {
    let stub = Stub::start();
    stub.reply(200, BULK);
    let ips = ["8.8.8.8", "1.1.1.1", "192.168.0.1", "9.9.9.9"];
    let rows = stub
        .client(KEY)
        .bulk(&ips)
        .await
        .unwrap()
        .data
        .unwrap()
        .response_array
        .unwrap();
    assert_eq!(rows.len(), 4);
    for (row, ip) in rows.iter().zip(ips) {
        assert_eq!(row.data.as_ref().unwrap().ip, ip);
    }

    let first = rows[0].data.as_ref().unwrap();
    assert!(rows[0].success);
    assert_eq!(
        first.geo_location.as_ref().unwrap().country.as_deref(),
        Some("United States")
    );
    assert_eq!(first.connection.as_ref().unwrap().asn_number, Some(15169));

    assert!(rows[1].success);
    assert!(rows[1].data.as_ref().unwrap().currency.is_none());

    assert!(!rows[2].success);
    assert!(rows[2].data.as_ref().unwrap().geo_location.is_none());
    assert_eq!(rows[2].message.as_deref(), Some("Reserved range/Invalid IP address"));

    assert!(!rows[3].success);
    assert!(rows[3].message.as_deref().unwrap().contains("quota"));
}

#[tokio::test]
async fn fields_and_raw_formats() {
    let stub = Stub::start();
    let client = stub.client(KEY);
    client
        .lookup("8.8.8.8", Some("json"), Some("geoLocation,timezone"))
        .await
        .unwrap();
    stub.reply(200, "ip,8.8.8.8");
    let raw = client.lookup("8.8.8.8", Some("csv"), None).await.unwrap();
    assert_eq!(raw.data.unwrap().ip, "ip,8.8.8.8");
    assert_eq!(
        stub.targets(),
        ["/ip/8.8.8.8?get=geoLocation%2Ctimezone", "/ip/8.8.8.8?format=csv"]
    );
}

#[tokio::test]
async fn http_errors_carry_the_status() {
    let stub = Stub::start();
    let client = stub.client(KEY);
    for status in [401u16, 404, 429] {
        stub.reply(status, r#"{"success":false,"message":"nope"}"#);
        match client.lookup("8.8.8.8", None, None).await {
            Err(IpWhoError::Api { status: got, message }) => {
                assert_eq!(got, StatusCode::from_u16(status).unwrap());
                assert_eq!(message, "nope");
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }
}

#[tokio::test]
async fn auth_errors_keep_the_api_error_code() {
    let stub = Stub::start();
    stub.reply(401, r#"{"error":"invalid_api_key"}"#);
    match stub.client(KEY).lookup("8.8.8.8", None, None).await {
        Err(IpWhoError::Api { message, .. }) => assert_eq!(message, "invalid_api_key"),
        other => panic!("unexpected result: {other:?}"),
    }
}

#[tokio::test]
async fn a_json_reply_is_parsed_even_when_another_format_was_asked_for() {
    // The API ignores `format` on /me and answers in JSON.
    let stub = Stub::start();
    let data = stub.client(KEY).me(Some("csv"), None).await.unwrap().data.unwrap();
    assert_eq!(data.ip, "8.8.8.8");
    assert_eq!(data.geo_location.unwrap().country.as_deref(), Some("United States"));
}

#[tokio::test]
async fn connection_failure_does_not_reveal_the_key() {
    let http = reqwest::Client::builder().no_proxy().build().unwrap();
    let client = IPWhoClient::with_client(KEY, http, Url::parse("http://127.0.0.1:1").unwrap());
    let error = client.lookup("8.8.8.8", None, None).await.unwrap_err();
    assert!(matches!(error, IpWhoError::Http(_)));
    assert!(!format!("{error} {error:?}").contains(KEY));
}

#[tokio::test]
async fn surrounding_whitespace_in_the_key_is_trimmed() {
    let stub = Stub::start();
    stub.client("  sk.test_secret_value\n")
        .lookup("8.8.8.8", None, None)
        .await
        .unwrap();
    let requests = stub.requests.lock().unwrap();
    assert_eq!(requests[0].authorization.as_deref(), Some("Bearer sk.test_secret_value"));
}

#[test]
fn a_bad_key_is_rejected_without_repeating_it() {
    let error = IPWhoClient::new("sk.secret_part\nmore").unwrap_err();
    assert!(!error.to_string().contains("secret_part"));
    assert!(IPWhoClient::new("  \n").is_err());
}
