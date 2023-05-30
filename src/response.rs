#[derive(Clone)]
pub struct Response(String);

pub static OK_STATUS: &str = "HTTP/1.1 200 OK\r\n";
pub static NOT_FOUND_STATUS: &str = "HTTP/1.1 404 Not Found\r\n";

impl Response {
    pub fn new(content: String, status: &str) -> Self {
        let mut response: String = String::new();
        response += status;

        let length = content.len();
        response += format!("Content-Length: {length}\r\n\r\n").as_str();
        response += content.as_str();
        Response(response)
    }
    pub fn bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
