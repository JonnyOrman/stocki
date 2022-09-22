use std::{net::TcpStream, io::{BufReader, BufRead}};

use crate::responses::Response;

pub struct Request{
    pub method: String,
    pub request_lines: Vec<String>
}

impl Request{
    fn new(
        method: String,
        request_lines: Vec<String> 
    ) -> Self{Self{
        method,
        request_lines
    }}
}

pub trait ReceiveRequest{
    fn receive(&self, stream: &TcpStream) -> Request;
}

pub struct RequestReceiver{}

impl RequestReceiver{
    pub fn new() -> Self{Self{}}
}

impl ReceiveRequest for RequestReceiver{
    fn receive(&self, mut stream: &TcpStream) -> Request{
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);

        let method = http_request[0].split(" ").collect::<Vec<&str>>()[0].to_string();

        return Request::new(
            method,
            http_request);
    }
}

pub trait HandleRequest{
    fn handle(&self, request: Request) -> Response;
}

pub trait HandleTcpStreamRequest {
    fn handle(&self, request: Request, stream: &TcpStream);
}
