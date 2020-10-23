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

This is the simpelest tide application that you can build but it is not very useful. It will return a 404 HTTP reply for any request. To be able to return anything else we will need to add one or more `Endpoint`s

== Endpoints ==

To make the `Server` return anything other than 404 we need to tell it how to react to requests. We do this
by adding Endpoints, functions that take a `Request` and return a `Response`.

```rust
server.at("/").get(|_req| async { Ok("Hello, world!") });
```

We use the `at` method to specify where the endpoint is added, the 

```rust
server.at("/").get(endpoint);

async fn endpoint(_req: tide::Request) -> Result {
    Ok("Hello, world!")
}
```