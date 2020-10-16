# Introduction

Tide is a minimal and pragmatic Rust web application framework built for
rapid development. It comes with a robust set of features that make building
async web applications and APIs easier and more fun.

## Getting started

In order to build a web app in Rust you need an HTTP server, and an async
runtime. After running `cargo init` add the following lines to your
`Cargo.toml` file:

```toml
# Example, use the version numbers you need
tide = "0.13.0"
async-std = { version = "1.6.0", features = ["attributes"] }
```

## Examples

Create an HTTP server that receives a JSON body, validates it, and responds
with a confirmation message.

```rust
use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
```

```sh
$ curl localhost:8000/orders/shoes -d '{ "name": "Chashu", "legs": 4 }'
Hello, Chashu! I've put in an order for 4 shoes

$ curl localhost:8000/orders/shoes -d '{ "name": "Mary Millipede", "legs": 750 }'
number too large to fit in target type
```
