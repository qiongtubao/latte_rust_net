
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use super::handler::Handler;
use std::net::{SocketAddr, ToSocketAddrs, TcpStream, Shutdown};
#[cfg(not(feature = "io_tokio"))]
use std::net::{TcpListener};
#[cfg(feature="io_tokio")]
use tokio;
#[cfg(feature="io_tokio")]
use tokio::io;
#[cfg(feature="io_tokio")]
use tokio::net::TcpListener;
use std::io::Write;
#[cfg(feature="io_tokio")]
use tokio::prelude::*;
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
    #[cfg(not(feature = "io_tokio"))]
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
    #[cfg(feature = "io_tokio")]
    pub fn listen(self, addr: &str) {
        let addr = addr.parse::<SocketAddr>().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();
        let done = listener
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            let handler =self.handler.clone();
            handler.accept(socket);
            Ok(())
        });
        tokio::run(done);

    }
}