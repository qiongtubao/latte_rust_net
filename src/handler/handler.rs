use std::net::{TcpStream};
pub trait Handler {
    // add code here
    fn accept(&self, mut istream: TcpStream);
}
