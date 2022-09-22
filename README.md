# stocki

Make tiny, single feature web services.


# Getting started

Create a new project:
```
cargo new stocki-example  
```

cd into it and install `stocki`:
```
cd stocki-example
cargo install stocki
```

Replace the contents of `src/main.rs` with this:
```
fn main() {
use stocki::{
    run,
    requests::{
        Request,
        HandleRequest
    },
    responses::{
        Response
    }
};

fn main() {
    let handler = RequestHandler::new();

    run(
        "localhost:7878".to_string(),
        "GET".to_string(),
        handler);
}

pub struct RequestHandler{}

impl RequestHandler {
    fn new() -> Self{Self{}}
}

impl HandleRequest for RequestHandler {
    fn handle(&self, _request: Request) -> Response{
        println!("stocki works!");
        return Response::new(200);
    }
}
```

Run the project:
```
cargo run
```

Make a `GET` request to `localhost:7878`. You should get a `200` response and see a "stocki works!" log message!