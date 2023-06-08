use once_cell::sync::Lazy;
use http_salient::Server;

fn main() {
    static mut server: Lazy<Server> = Lazy::new(|| Server::new());

    // It's safe because we don't change server in multiple threads.
    unsafe { server.run() }
}
