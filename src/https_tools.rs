pub struct Response(String);

const OK_STATUS: &str = "HTTP/1.1 200 OK\r\n";

impl Response {
    pub fn new(content: String) -> Self {
        let mut response: String = String::new();
        response += OK_STATUS;

        let length = content.len();
        response += format!("Content-Length: {length}\r\n\r\n").as_str();
        response += content.as_str();
        Response(response)
    }
    pub fn bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
