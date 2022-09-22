# stocki

Make tiny, single feature web services.


# Getting started

Create a new project:
```
cargo new stocki-example  
```

cd into the new project directory:
```
cd stocki-example
```

Add the following under `[dependencies]` in `Cargo.toml` :
```
stocki = "0.1.0"
```

Replace the contents of `src/main.rs` with this:
```
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