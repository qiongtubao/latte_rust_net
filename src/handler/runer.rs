use tokio::io::{AsyncRead, AsyncWrite};
use futures::{try_ready, Future, Poll};
use std::str;
use std::io;
#[derive(Debug)]
pub struct Runer<R,W,H> {
    reader: Option<R>,
    read_done: bool,
    writer:  Option<W>,
    handler: Box<H>,
    pos: usize,
    cap: usize,
    amt: u64,
    buf: Box<[u8]>,
}

pub fn runer<R,W,H>(reader: R, writer: W, handler: Box<H>) -> Runer<R,W,H> 
where 
    R: AsyncRead,
    W: AsyncWrite,
    H: Fn(String) -> Vec<String>,
{
    Runer {
        reader: Some(reader),
        read_done: false,
        writer: Some(writer),
        handler: handler,
        amt: 0,
        pos: 0,
        cap: 0,
        buf: Box::new([0; 2048]),
    }
}
impl<R, W, H> Future for Runer<R, W, H>
where 
    R: AsyncRead,
    W: AsyncWrite, 
    H: Fn(String) -> Vec<String>,
{
    type Item = (u64, R, W);
    type Error = io::Error;
    fn poll(&mut self) -> Poll<(u64, R, W), io::Error> {
        loop {
            if !self.read_done {
                let reader = self.reader.as_mut().unwrap();
                let n = try_ready!(reader.poll_read(&mut self.buf));
                if n == 0 {
                    self.read_done = true;
                } else {
                    self.pos = 0;
                    self.cap = n;
                }
            }

           
            let writer =  self.writer.as_mut().unwrap();
            let s = match str::from_utf8(&self.buf[self.pos..self.cap]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            let datas = (self.handler)(s.to_string());
            for data in datas {
                let bytes = data.as_bytes();
                let len = bytes.len();
                let mut k = 0;
                //确保发送成功
                while k < len {
                    let i = try_ready!(writer.poll_write(&bytes[k..len]));
                    if i == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            "write zero byte into writer",
                        ));
                    }
                    k += i;
                    self.amt += i as u64;
                }
            }
                
            
            if self.read_done {
                try_ready!(self.writer.as_mut().unwrap().poll_flush());
                let reader = self.reader.take().unwrap();
                let writer = self.writer.take().unwrap();
                return Ok((self.amt, reader, writer).into())
            }
        }
    }
}