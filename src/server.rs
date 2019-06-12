
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use super::handler::Handler;
use std::net::{SocketAddr, ToSocketAddrs, TcpStream, TcpListener, Shutdown};
use std::io::Write;
use std::str;

pub struct Server<F> where
F: Handler + 'static + Send + Sync,{
    handler: Arc<F> ,
}
impl<F> Server<F> where
F: Handler + 'static + Send + Sync,  {
    pub fn new(handler: F) -> Server<F> {
        Server {
            handler: Arc::new(handler),
        }
    }
    pub fn listen(self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        let a = thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut istream) => {
                        let handler =self.handler.clone();
                        let t = thread::spawn(move || {
                            handler.accept(istream);
                        });
                        // handler.accept(istream);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        });
        a.join();
    }
}