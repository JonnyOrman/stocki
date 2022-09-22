use std::net::TcpStream;

use crate::{requests::{ReceiveRequest, HandleTcpStreamRequest, HandleRequest, Request}, responses::{WriteResponse, Response}};

pub trait Handle {
    fn handle(&self, stream: TcpStream);
}

pub struct ConnectionHandler<TRequester: ReceiveRequest, TTcpStreamHandler : HandleTcpStreamRequest>{
    request_receiver: TRequester,
    request_handler: TTcpStreamHandler
}

impl<TRequester: ReceiveRequest, TTcpStreamHandler : HandleTcpStreamRequest> ConnectionHandler<TRequester, TTcpStreamHandler> {
    pub fn new(
        request_receiver: TRequester,
        request_handler: TTcpStreamHandler
    ) -> Self{
        Self {
            request_receiver,
            request_handler
        }
    }
}

impl<TRequester: ReceiveRequest, TTcpStreamHandler : HandleTcpStreamRequest> Handle for ConnectionHandler<TRequester, TTcpStreamHandler>{
    fn handle(&self, stream: TcpStream) {
        let request = self.request_receiver.receive(&stream);

        self.request_handler.handle(request, &stream);
    }
}


pub struct TcpStreamHandler<TRequestHandler: HandleRequest, TResponseWriter: WriteResponse> {
    request_handler: TRequestHandler,
    response_writer: TResponseWriter,
    method: String
}

impl<TRequestHandler: HandleRequest, TResponseWriter: WriteResponse> TcpStreamHandler<TRequestHandler, TResponseWriter>{
    pub fn new(
        request_handler: TRequestHandler,
        response_writer: TResponseWriter,
        method: String
    ) -> Self{Self{
        request_handler,
        response_writer,
        method
    }}
}

impl<TRequestHandler: HandleRequest, TResponseWriter: WriteResponse> HandleTcpStreamRequest for TcpStreamHandler<TRequestHandler, TResponseWriter>{
    fn handle(&self, request: Request, stream: &TcpStream){
        let response: Response;

        if request.method == self.method {
            response = self.request_handler.handle(request);
        }
        else {
            response = Response::new(405);
        }

        self.response_writer.write(response, &stream);
    }
}