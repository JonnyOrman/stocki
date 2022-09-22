use requests::HandleRequest;

pub mod applications;
pub mod requests;
pub mod responses;
pub mod connections;

pub fn run<TRequestHandler: HandleRequest>(
    address_binding: String,
    method: String,
    request_handler: TRequestHandler) {
    let application = applications::build_application(
        address_binding,
        method,
        request_handler);

    application.run();
}
