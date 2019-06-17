use latte_rust_net::{Server, TcpHandler};

fn main() {
    let handler = TcpHandler::new(|mut data:String| -> Vec<String> {
        data.insert_str(0, "server:");
        vec![data]
    });
    let server = Server::new(handler);
    server.listen("127.0.0.1:3333");
}