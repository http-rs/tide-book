# The server, Routes and Endpoints

The central part of a Tide application is the `Server` struct. A Tide application is started by creating a
`Server` and configuring it with `Route`s to `Endpoint`s.
When a `Server` is started it will handle incoming `Request`s by matching their URLs with Routes. Requests that match a route are then  dispatched to the corresponding `Endpoint`.

## Set up a Server

A Tide `Server` can simply be constructed with its `new()` constructor.
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

While this is the simpelest Tide application that you can build, it is not very useful. It will return a 404 HTTP response to any request. To be able to return anything useful we will need to handle requests using one or more `Endpoint`s

## Handle requests with Endpoints

To make the `Server` return anything other than an HTTP 404 reply we need to tell it how to react to requests. We do this by adding one or more Endpoints;

```rust
server.at("*").get(|_req| async { Ok("Hello, world!") });
```

We use the `at` method to specify the route to the endpoint. We will talk about routes later. For now we'll just use the `"*"` wildcard route that matches anything we throw at it. For this example we will add an async closure as the `Endpoint`. Tide expects something that implements the `Endpoint` trait here. But this closure will work because Tide implements the `Endpoint` trait for certain async functions with a signature that looks like this;
```rust
async fn endpoint(request: Request) -> Result<impl Into<Response>>
```

In this case `Into<Response>` is implemented for `&str` so our closure is a valid Endpoint. Because `Into<Response>` is implemented for several other types you can quickly set up endpoints. For example the next endpoint uses the `json!` macro from the `serde_json` crate to return a `serde_json::Value`.

```rust
server.at("*").get(|req| async {
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
server.at("*").get(|req| async {
    Ok(Response::new(StatusCode::Ok).set_body("Hello world".into()))
});
```

The `Response` type is described in more detail in the next chapter.

More than one endpoint can be added by chaining methods. For example if we want to reply to a `delete` request as wel as a `get` request endpoints can be added for both;

```rust
server.at("*")
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

## Set up advanced endpoints by implementing the `Endpoint` trait

TODO

## Set up simple routes

The server we build is still pretty lame. It will return the same response to any URL it sees. It is only able to differentiate between requests by HTTP method. But we already used the `.at` method of the `Server` to define a wildcard route. You might already have guessed how to define more complicated routes.

TODO: expand this

## Compose routes

TODO

## Match routes using wildcards

TODO
