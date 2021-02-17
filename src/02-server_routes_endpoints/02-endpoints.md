# Handle requests with endpoints

To make the `Server` return anything other than an HTTP 404 reply we need to tell it how to react to requests. We do this by adding one or more Endpoints;

```rust,edition2018,no_run
#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut server = tide::new();
    server.at("*").get(|_| async { Ok("Hello, world!") });
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
```

We use the `at` method to specify the route to the endpoint. We will talk about routes later. For now we'll just use the `"*"` wildcard route that matches anything we throw at it. For this example we will add an async closure as the `Endpoint`. Tide expects something that implements the `Endpoint` trait here. But this closure will work because Tide implements the `Endpoint` trait for certain async functions with a signature that looks like this;

```rust,ignore
async fn endpoint(request: tide::Request) -> tide::Result<impl Into<Response>>
```

In this case `Into<Response>` is implemented for `&str` so our closure is a valid Endpoint. Because `Into<Response>` is implemented for several other types you can quickly set up endpoints. For example the next endpoint uses the `json!` macro provided by `use tide::prelude::*` to return a `serde_json::Value`.

```rust,edition2018,no_run
use tide::prelude::*;
#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut server = tide::new();
    server.at("*").get(|_| async {
        Ok(json!({
            "meta": { "count": 2 },
            "animals": [
                { "type": "cat", "name": "chashu" },
                { "type": "cat", "name": "nori" }
            ]
        }))
    });
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
```

Returning quick string or json results is nice for getting a working endpoint quickly. But for more control a full `Response` struct can be returned.

```rust,ignore
server.at("*").get(|_| async {
    Ok(Response::new(StatusCode::Ok).set_body("Hello world".into()))
});
```

The `Response` type is described in more detail in the next chapter.

More than one endpoint can be added by chaining methods. For example if we want to reply to a `delete` request as well as a `get` request endpoints can be added for both;

```rust,ignore
server.at("*")
    .get(|_| async { Ok("Hello, world!") })
    .delete(|_| async { Ok("Goodbye, cruel world!") });
```

Eventually, especially when our endpoint methods grow a bit, the route definitions will get a crowded. We could move our endpoint implementations to their own functions;

```rust,edition2018,no_run
#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut server = tide::new();
    server.at("*").get(endpoint);
    server.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn endpoint(_req: tide::Request<()>) -> Result<Response> {
    Ok(Response::new(StatusCode::Ok).set_body("Hello world".into()))
}
```