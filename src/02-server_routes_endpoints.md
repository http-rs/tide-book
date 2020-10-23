# The server, Routes and Endpoints

The central part of a Tide application is the `Server` struct. A Tide application is started by creating a
`Server` and configuring it with `Route`s to `Endpoint`s. And then starting it.
When a `Server` is started it will handle incoming `Request`s by matching their URLs with Routes and dispatching every `Request` to the right `Endpoint`

== Server ==

A Tide `Server` can simply be constructed with the `new()` constructor.
```rust
use tide::prelude::*;
let mut = Server::new();
```
or even shorter by using the `new()` convenience method provided in the `tide` namespace
```rust
let mut server = tide::new();
```

The server can then be started using the asynchronous `listen` method.
```rust
let mut server = tide::new();
server.listen("127.0.0.1:8080").await?;
```

This is the simpelest tide application that you can build but it is not very useful. It will only return a 404 HTTP replies. To be able to return anything else we will need to add one or more `Endpoint`s

== Endpoints ==

To make the `Server` return anything other than a 404 reply we need to tell it how to react to requests. We do this
by adding one or more Endpoints;

```rust
server.at("/").get(|_req| async { Ok("Hello, world!") });
```

We use the `at` method to specify the route to the endpoint. The endpoint itself is just an async closure. This will work because Tide implements the 'Endpoint' trait for certain async functions with a signature that looks like this.

```rust
async fn endpoint(request: Request) -> Result<impl Into<Response>>
```

In this case `Into<Response>` is implemented for `&str` so our closure is a valid Endpoint. Because `Into<Response>` is implemented for several other types you can quickly set up useful endpoints. For example this endpoint uses the `json!` from the `serde_json` crate to return a `serde_json::Value`.

```rust
server.at("/").get(|req| async {
    Ok(json!({
        "meta": { "count": 2 },
        "animals": [
            { "type": "cat", "name": "chashu" },
            { "type": "cat", "name": "nori" }
        ]
    }))
})
```

Returning quick string or json results is nice for getting a working endpoint quickly. But for more control a full `Response` struct can be returned.

```rust
server.at("/").get(|req| async {
    Ok(Response::new(StatusCode::Ok).set_body("Hello world".into()))
});
```

The `Response` is described in more detail in the next chapter.

More than one endpoint can be added by chaining methods. For example if we want to reply to a `delete` request as wel as a `get request endpoints can be added for both;

```rust
server.at("/")
    .get(|_req| async { Ok("Hello, world!") })
    .delete(|_req| async { Ok("Goodbye, cruel world!") });
```

Eventually, especially when our endpoint methods grow a bit, the route definitions will get a crowded. We could move our endpoint implementations to their own functions;

```rust
server.at("/").get(endpoint);

async fn endpoint(_req: tide::Request) -> Result<Response> {
    Ok(Response::new(StatusCode::Ok).set_body("Hello world".into()))
}
```
