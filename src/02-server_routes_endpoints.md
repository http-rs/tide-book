# The server, Routes and Endpoints

The central part of a Tide application is the `Server` struct. A Tide application is started by creating a
`Server` and configuring it with `Route`s to `Endpoint`s. And then starting it.
When a `Server` is started it will handle incoming `Request`s by matching their URLs with Routes and dispatching every `Request` to the right `Endpoint`

== Server ==

A Tide `Server` can simply be constructed with the `new()` constructor.
```rust
use tide::prelude::*
let mut = Server::new()
```
or even shorter by using the `new()` convenience method provided in the `tide` namespace
```rust
let mut server = tide::new()
```

The server can then be started using the asynchronous `listen` method.
```rust
let mut server = tide::new()
server.listen("127.0.0.1:8080").await?;
```

This is the simpelest tide application that you can build but it is not very useful. It will return a 404 HTTP reply for any request. To be able to return anything else we will need to add one or more `Endpoint`s

== Endpoint ==


== Route ==

=== Defining routes ===

=== Route parameters ===
