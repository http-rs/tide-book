# Getting started

In order to build a web app in Rust you need an HTTP server, and an async
runtime. After running `cargo new --bin web-app` add the following lines to your
`Cargo.toml` file:

```toml
# Example, use the version numbers you need
tide = "0.15.0"
async-std = { version = "1.6.5", features = ["attributes"] }
```

# Example

Create an HTTP server that receives a JSON body, validates it, and responds
with a confirmation message.

```rust,edition2018,no_run
{{#include ../../examples/ch01-02-example/src/main.rs:example}}
```

```sh
$ curl localhost:8080/orders/shoes -d '{ "name": "Chashu", "legs": 4 }'
```

Hello, Chashu! I've put in an order for 4 shoes

```sh
$ curl localhost:8080/orders/shoes -d '{ "name": "Mary Millipede", "legs": 750 }'
```

number too large to fit in target type

