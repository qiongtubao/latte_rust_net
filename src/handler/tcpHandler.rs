
extern crate bytes;
use super::Handler;
use std::io as std_io;
#[cfg(not(feature = "io_tokio"))]
use std::net::{TcpStream};
#[cfg(feature = "io_tokio")]
use tokio::net::TcpStream;
#[cfg(feature = "io_tokio")]
use tokio::io;
#[cfg(feature = "io_tokio")]
use futures::future::Future;
#[cfg(feature = "io_tokio")]
use tokio::prelude::*;
use std::io::BufReader;
use futures::{try_ready, Poll,Async};
use std::sync::Arc;
// use futures::poll::Async;
//虽然这个现在知识tokio 用之后应该可以用到普通stream上
use bytes::{BytesMut, Bytes, BufMut};
use std::net::{Shutdown};
use std::io::Read;
use std::iter;
use std::str;
use std::io::Write;
#[derive(Clone)]
pub struct TcpHandler<T>  {
    handle: Box<T>,
}

impl<T> TcpHandler<T> where 
    T: Fn(String) -> Vec<String> + 'static + Send  + Sync + Clone, {
    pub fn new(t: T) -> TcpHandler<T> {
        TcpHandler {
            handle: Box::new(t),
        }
    }
}

impl<T> Handler for TcpHandler<T> where 
    T: Fn(String) -> Vec<String> + 'static + Send  + Sync + Clone, {
    #[cfg(not(feature = "io_tokio"))]
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
    #[cfg(feature = "io_tokio")]
    fn accept(&self, mut istream: TcpStream) {
        let (reader, writer) = istream.split();
        let handle = self.handle.clone();
        let amt = super::runer(reader, writer, handle);
        let msg = amt.then(move |result| {
            match result {
                Ok((amt, _, _)) => println!("wrote {} bytes", amt),
                Err(e) => println!("error: {}", e),
            }
            Ok(())
        });
        tokio::spawn(msg);
    }
}