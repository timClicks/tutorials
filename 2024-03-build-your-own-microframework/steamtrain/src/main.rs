use std::io;
use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

/// End user
struct Client;

/// Computer hosting the web app
struct Server {
    connection: TcpListener,
}

impl Server {
    fn new(address: &str) -> Server {
        let listener = TcpListener::bind(address).unwrap();

        Server {
            connection: listener
        }
    }
}

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

type Headers = std::collections::HashMap<String, Vec<String>>;

/// Sent from the Client
#[derive(Debug)]
struct Request {
    // version: // 0.9, 1.0, 1.1, 2.0

    /// Represents matching routes to things that
    /// our server might know about
    resource: String,

    method: HttpMethod,

    headers: Headers,

    body: Vec<u8>,
}

fn read_header_line(stream: &mut BufReader<TcpStream>) -> io::Result<String> {
    let mut buf: Vec<u8> = Vec::with_capacity(0x1000);

    while let Some(Ok(byte)) = stream.bytes().next() {
        if byte == b'\n' {
            if buf.ends_with(b"\r") {
                buf.pop();
            }

            let header_line = String::from_utf8(buf)
                    .map_err(|_| { io::Error::new(io::ErrorKind::InvalidData, "Not an HTTP header") })?;
            return Ok(header_line);
        }

        buf.push(byte);
    }

    Err(io::Error::new(io::ErrorKind::ConnectionAborted, "client aborted early"))
}


impl Request {
    fn new(mut stream: BufReader<TcpStream>) -> io::Result<Request> {

        // GET /index.html HTTP/1.1
        //  ^  ^                ^ version
        //  |  \ resource
        //  |
        //  \ method
        let http_metadata = read_header_line(&mut stream)?;

        eprintln!("{http_metadata}");

        let mut parts = http_metadata.split_ascii_whitespace();

        let method = match parts.next().unwrap() {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "DELETE" => HttpMethod::Delete,
            "PUT" => HttpMethod::Put,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "unsupported HTTP method"))
        };

        let resource = parts.next().unwrap().to_string();

        let _version = parts.next();

        let mut headers = Headers::new();

        loop {
            let line = read_header_line(&mut stream)?;
            if line.is_empty() {
                break;
            }

            let mut parts = line.split(": ");
            let name /* Content-Type */ = parts.next().unwrap().to_string();
            let value /* text/html */ = parts.next().unwrap().to_string();

            let slot_for_value = headers
                .entry(name)
                .or_insert_with(|| { Vec::with_capacity(1) });
            slot_for_value.push(value);

        }

        let mut body = Vec::with_capacity(0x10000);
        let _ = stream.read(&mut body)?;

        Ok(Request {
            resource,
            method,
            headers,
            body,
        })
    }
}

/// Sent from the Server
struct Response;


fn main() {
    let server = Server::new("0.0.0.0:8080");

    for stream in server.connection.incoming().flatten()
        .map(|s| {
            let _ = s.set_read_timeout(Some(Duration::from_secs(1)));
            BufReader::new(s)}
        ) {
        if let Ok(req) = Request::new(stream) {
            // Response::new(req);
            println!("{req:?}");
        };
    }
}
