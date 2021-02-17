# Set up a Server

A basic Tide `Server` is constructed with `tide::new()`.

```rust,edition2018,no_run
#[async_std::main]
async fn main() -> tide::Result<()> {
    let server = tide::new();
    Ok(())
}
```

The server can then be started using the asynchronous `listen` method.

```rust,edition2018,no_run
#[async_std::main]
async fn main() -> tide::Result<()> {
    let server = tide::new();
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
```

While this is the simpelest Tide application that you can build, it is not very useful. It will return a 404 HTTP response to any request. To be able to return anything useful we will need to handle requests using one or more `Endpoint`s