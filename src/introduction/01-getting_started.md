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
Hello, Chashu! I've put in an order for 4 shoes
```

Let's try now with an invalid number of legs (**Note** that we use the `-v` flag to use curl in verbose mode).

```sh
$ curl -v localhost:8080/orders/shoes -d '{ "name": "Mary Millipede", "legs": 750 }'
< HTTP/1.1 422 Unprocessable Entity
< content-length: 0
< date: Fri, 26 Feb 2021 13:31:17 GMT
```

We get an http error, 422 to be more specific, and that is because we are using the `body_json` method to `deseriaize` the body into the `Animal` struct and the `legs` fields type is `u8`. We will cover `body_json` and the other available methods for `deserialize` the body in the [Request/Response chapter](../03-request-response/01-request.md).
