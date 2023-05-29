use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use config::Config;
mod config;

use https_tools::{Response, NOT_FOUND_STATUS, OK_STATUS};
mod https_tools;
pub struct Server {
    listener: TcpListener,
    config: Config,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        let config: Config = confy::load("salient", None).unwrap();
        let listener = TcpListener::bind(&config.address).unwrap();
        Server { listener, config }
    }

    pub fn run(&mut self) {
        let double_dot_defence = self.config.double_dot_defence;

        for stream in self.listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => continue,
            };
            thread::spawn(move || {
                Self::handle_connection(stream, double_dot_defence);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, double_dot_defence: bool) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut path = http_request[0].split(' ').nth(1).unwrap_or("/");

        if double_dot_defence && path.contains("..") {
            path = "not_found";
        }

        if path == "/" {
            path = "/index.html";
        }

        let temp_path = if path.contains('.') {
            format!("./www{path}")
        } else {
            format!("./www{path}.html")
        };

        path = temp_path.as_str();

        let response = match fs::read_to_string(path) {
            Ok(result) => Response::new(result, OK_STATUS),
            Err(_) => Response::new(
                fs::read_to_string("./www/not_found.html").unwrap_or("Not found.".to_string()),
                NOT_FOUND_STATUS,
            ),
        };

        let _ = stream.write_all(response.bytes());
    }
}
