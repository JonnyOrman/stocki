use std::net::TcpListener;

use crate::{requests::{ReceiveRequest, RequestReceiver, HandleTcpStreamRequest}, HandleRequest, responses::ResponseWriter, connections::{ConnectionHandler, TcpStreamHandler, Handle}};


pub struct Application<TRequester: ReceiveRequest, TTcpStreamHandler : HandleTcpStreamRequest>{
    address_binding: String,
    connection_handler: ConnectionHandler<TRequester, TTcpStreamHandler>
}

impl<TRequester: ReceiveRequest, TTcpStreamHandler : HandleTcpStreamRequest> Application<TRequester, TTcpStreamHandler> {
    pub fn new(
        address_binding: String,
        connection_handler: ConnectionHandler<TRequester, TTcpStreamHandler>) -> Self{
        Self { 
            connection_handler,
            address_binding}
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.address_binding.clone()).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.connection_handler.handle(stream);
        }
    }
}

pub fn build_application<TRequestHandler: HandleRequest>(
    address_binding: String,
    method: String,
    request_handler: TRequestHandler) -> Application<RequestReceiver, TcpStreamHandler<TRequestHandler, ResponseWriter>>{
    let receiver = RequestReceiver::new();

    let response_writer = ResponseWriter::new();

    let handler = TcpStreamHandler::new(
        request_handler,
        response_writer,
        method
    );

    let connection_handler = ConnectionHandler::new(
        receiver,
        handler
    );

    let application = Application::new(
        address_binding.to_string(),
        connection_handler);

    return application;
}
