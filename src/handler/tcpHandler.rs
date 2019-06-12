
use super::Handler;
use std::net::{Shutdown, TcpStream};
use std::io::Read;
use std::str;
use std::io::Write;

pub struct TcpHandler<T> where
    T: Fn(String) -> Vec<String> + 'static + Send  + Sync, {
    handle: Box<T>,
}

impl<T> TcpHandler<T> where 
    T: Fn(String) -> Vec<String> + 'static + Send  + Sync, {
    pub fn new(t: T) -> TcpHandler<T> {
        TcpHandler {
            handle: Box::new(t),
        }
    }
}

impl<T> Handler for TcpHandler<T> where 
    T: Fn(String) -> Vec<String> + 'static + Send  + Sync, {
    fn accept(&self, mut istream: TcpStream) {
        let mut data = [0 as u8; 50];
        while match istream.read(&mut data) {
            Ok(size) => {
                let s = match str::from_utf8(&data[0..size]) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                let datas = (self.handle)(s.to_string());
                for data in datas {
                    istream.write(data.as_bytes()).unwrap();
                }
                true
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", istream.peer_addr().unwrap());
                istream.shutdown(Shutdown::Both).unwrap();
                false
            }
        }{ }
    }
}