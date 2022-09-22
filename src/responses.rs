use std::{net::TcpStream, io::Write};

pub struct Response{
    status: u16
}

impl Response {
    pub fn new(status: u16) -> Self{Self{status}}
}

pub trait WriteResponse{
    fn write(&self, response: Response, stream: &TcpStream);
}

pub struct ResponseWriter{}

impl ResponseWriter{
    pub fn new() -> Self{Self{}}
}

impl WriteResponse for ResponseWriter{
    fn write(&self, response: Response, mut stream: &TcpStream){
        let response = format!("HTTP/1.1 {}\r\n\r\n{}", response.status, "");

        stream.write_all(response.as_bytes()).unwrap();
    }
}