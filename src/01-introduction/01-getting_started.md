# Getting started

In order to build a web app in Rust you need an HTTP server, and an async
runtime. After running `cargo init` add the following lines to your
`Cargo.toml` file:

```toml
# Example, use the version numbers you need
tide = "0.13.0"
async-std = { version = "1.6.0", features = ["attributes"] }
```