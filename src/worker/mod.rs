use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::path::Path;

#[derive(Debug)]
pub struct Worker {
    stream: TcpStream,
    id: u32,
    base_path: String,
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
    UNSUPPORTED,
}

impl Worker {
    pub fn new(stream: TcpStream, base: String, id: u32) -> Self {
        Self {
            stream,
            id,
            base_path: base,
        }
    }

    fn clean<'a>(&'a self, request_file: &'a str) -> &str {
        if request_file.len() > 1 && request_file.starts_with("/") {
            return &request_file[1..];
        }

        request_file
    }

    pub fn handle_request(&mut self) {
        println!("Handling request on Worker {}", self.id);

        let buf_reader = BufReader::new(&mut self.stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let method_request: Vec<&str> = http_request[0].split_whitespace().collect();
        let request_file = self.clean(method_request[1].clone());

        let method = match method_request[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::UNSUPPORTED,
        };

        let response = match method {
            Method::GET => self.process_get(request_file),
            Method::POST => self.process_post(),
            Method::UNSUPPORTED => self.process_unsupported(Method::UNSUPPORTED),
        };

        self.stream.write_all(response.as_bytes()).unwrap();
    }

    fn process_get(&self, request_file: &str) -> String {
        let path = Path::new(self.base_path.as_str());

        let file = if request_file == "/" {
            "index.html"
        } else {
            request_file
        };

        let full_path = path.join(file);

        if path.exists() {
            println!("\tGetting {}", full_path.display());

            let status_line = "HTTP/1.1 200 OK";

            if let Ok(contents) = fs::read_to_string(&full_path) {
                let length = contents.len();
                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

                return response;
            }
        }
        self.process_404(full_path.display().to_string())
    }

    fn process_404(&self, full_path: String) -> String {
        let status_line = "HTTP/1.1 404 NOT FOUND";

        println!("\tFile not found: {}", full_path);

        let not_found_html = vec![
            "<!DOCTYPE html>\n",
            "<html lang=\"en\">\n",
            "<head>\n",
            "\t<meta charset=\"utf-8\">\n",
            "\t<title>Hello!</title>\n",
            "</head>\n",
            "<body>\n",
            "\t<h1>Oops!</h1>\n",
            "\t<p>Sorry, I don't know what you're asking for.</p>\n",
            "</body>\n",
            "</html>\n",
        ];

        let length: usize = not_found_html.iter().map(|s| s.len()).sum();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{}",
            not_found_html.join("")
        );

        response
    }

    fn process_post(&self) -> String {
        self.process_unsupported(Method::POST)
    }

    fn process_unsupported(&self, method: Method) -> String {
        let status_line = "HTTP/1.1 501 NOT IMPLEMENTED";

        let method_str = format!("\t<h1>Method: {:?}</h1>\n", method);
        println!("\tMethod not supported: {:?}", method);

        let not_implemented_html = vec![
            "<!DOCTYPE html>\n",
            "<html lang=\"en\">\n",
            "<head>\n",
            "\t<meta charset=\"utf-8\">\n",
            "\t<title>Hello!</title>\n",
            "</head>\n",
            "<body>\n",
            method_str.as_str(),
            "\t<p>Sorry, this functionality has not been implemented yet.</p>\n",
            "</body>\n",
            "</html>\n",
        ];

        let length: usize = not_implemented_html.iter().map(|s| s.len()).sum();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{}",
            not_implemented_html.join("")
        );

        response
    }
}
