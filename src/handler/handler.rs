#[cfg(not(feature = "io_tokio"))]
use std::net::{TcpStream};
#[cfg(feature = "io_tokio")]
use tokio::net::TcpStream;
#[cfg(feature = "io_tokio")]
use futures::{Poll};

use std::io;
pub trait Handler {
    // add code here
    #[cfg(not(feature = "io_tokio"))]
    fn accept(&self, mut istream: TcpStream);
    #[cfg(feature = "io_tokio")]
    fn accept(&self, mut istream: TcpStream);
}

