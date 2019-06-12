use latte_rust_net::{Server, TcpHandler};

fn main() {
    let handler = TcpHandler::new(|data:String| -> Vec<String> {
        vec![data]
    });
    let server = Server::new(handler);
    server.listen("127.0.0.1:3333");
}