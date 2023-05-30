use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use config::Config;
mod config;

use response::{Response, NOT_FOUND_STATUS, OK_STATUS};
mod response;

mod page;

use cache::Cache;
mod cache;

pub struct Server {
    listener: TcpListener,
    config: Config,
    cache: Option<Cache>,
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

        let cache: Option<Cache> = if config.caching {
            Some(Cache::new())
        } else {
            None
        };

        Server {
            listener,
            config,
            cache,
        }
    }

    pub fn run(&mut self) {
        let double_dot_defence = self.config.double_dot_defence;
        for stream in self.listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(_) => continue,
            };

            let cache = self.cache.clone();
            thread::spawn(move || {
                Self::handle_connection(stream, double_dot_defence, cache);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, double_dot_defence: bool, cache: Option<Cache>) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut path = http_request[0].split(' ').nth(1).unwrap_or("/");
        let path = Self::format_path(&mut path, &double_dot_defence);

        let response: Response = match cache {
            None => match fs::read_to_string(path) {
                Ok(result) => Response::new(result, OK_STATUS),
                Err(_) => Response::new(
                    fs::read_to_string("./www/not_found.html").unwrap_or("Not found.".to_string()),
                    NOT_FOUND_STATUS,
                ),
            },
            Some(cache) => {
                let mut demanded_page = None;
                for page in cache.pages {
                    if page.path == path {
                        demanded_page = Some(page.response);
                    }
                }
                demanded_page.unwrap_or(cache.not_found)
            }
        };

        let _ = stream.write_all(response.bytes());
    }

    fn format_path<'a>(mut path: &str, double_dot_defence: &bool) -> String {
        if *double_dot_defence && path.contains("..") {
            path = "not_found";
        }

        if path == "/" {
            path = "/index.html";
        }

        if path.contains('.') {
            format!("./www{path}")
        } else {
            format!("./www{path}.html")
        }
    }
}
