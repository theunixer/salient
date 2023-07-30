use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use config::Config;
mod config;

use response::Response;
mod response;

mod page;

use cache::Cache;
mod cache;

mod content;

pub struct Server {
    listener: TcpListener,
    config: Config,
    cache: Option<Cache>,
    requests_handled: u128,
}

impl Server {
    pub fn new() -> Self {
        let config: Config = confy::load("salient", None).unwrap();
        let listener = TcpListener::bind(&config.address).unwrap();

        let cache: Option<Cache> = if config.caching {
            Some(Cache::new())
        } else {
            None
        };

        let requests_handled = 0u128;

        Server {
            listener,
            config,
            cache,
            requests_handled,
        }
    }

    pub fn run(&'static mut self) {
        println!(
            "Starting an http server at http://{}",
            self.listener.local_addr().unwrap().to_string()
        );
        let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
        let output_delay = self.config.statistics_output_delay as u128;
        for stream in self.listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => continue,
            };

            if self.config.statistics {
                self.requests_handled += 1;
                if self.requests_handled % output_delay == 0 {
                    println!("Requests handled: {}", self.requests_handled);
                }
            }

            loop {
                if threads.len() < self.config.thread_limit {
                    break;
                } else {
                    let _ = threads.remove(0).join();
                }
            }

            threads.push(thread::spawn(|| {
                Self::handle_connection(stream, &self.config.double_dot_defence, &self.cache);
            }));
        }
    }

    fn handle_connection(mut stream: TcpStream, double_dot_defence: &bool, cache: &Option<Cache>) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let request_type = http_request[0].split(' ').nth(0).unwrap_or("GET");
        let path = http_request[0].split(' ').nth(1).unwrap_or("/");

        if request_type == "GET" {
            let path = content::format_path(path, &double_dot_defence);
            let response: Response = match cache {
                None => content::page_from_file(&path),
                Some(cache) => content::page_from_cache(&cache, &path),
            };
            let _ = stream.write_all(response.bytes());
        } else if request_type == "POST" {
            match path {
                _ => {}
            }
        }
    }
}
