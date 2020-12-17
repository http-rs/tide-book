# The server, Routes and Endpoints

The central part of a Tide application is the `Server` struct. A Tide application is started by creating a
`Server` and configuring it with `Route`s to `Endpoint`s.
When a `Server` is started it will handle incoming `Request`s by matching their URLs with Routes. Requests that match a route are then  dispatched to the corresponding `Endpoint`.

## Set up a Server

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

## Handle requests with endpoints

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

```rust,edition2018,no_run
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

```rust,edition2018,no_run
server.at("*").get(|_| async {
    Ok(Response::new(StatusCode::Ok).set_body("Hello world".into()))
});
```

The `Response` type is described in more detail in the next chapter.

More than one endpoint can be added by chaining methods. For example if we want to reply to a `delete` request as wel as a `get` request endpoints can be added for both;

```rust,edition2018,no_run
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

## Defining and composing routes

The server we built is still not very useful. It will return the same response for any URL. It is only able to differentiate between requests by HTTP method. We already used the `.at` method of the `Server` to define a wildcard route. You might have guessed how to add endpoints to specific routes;

```rust,edition2018,no_run
#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut server = tide::new();

    server.at("/hello").get(|_| async { Ok("Hello, world!") });
    server.at("/bye").get(|_| async { Ok("Bye, world!") });

    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
```

Here we added two routes for two different endpoints. Routes can also be composed by chaining the `.at` method.

```rust,edition2018,no_run
server.at("/hello").at("world").get(|_| async { Ok("Hello, world!") });
```
This will give you the same result as:

```rust,edition2018,no_run
server.at("/hello/world").get(|_| async { Ok("Hello, world!") });
```

We can store the partial routes and re-use them;

```rust,edition2018,no_run
#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut server = tide::new();

    let hello_route = server.at("/hello");

    hello_route.get(|_| async { Ok("Hi!") });
    hello_route.at("world").get(|_| async { Ok("Hello, world!") });
    hello_route.at("mum").get(|_| async { Ok("Hi, mum!") });

    server.listen("127.0.0.1:8080").await?;
    Ok(())
}
```
Here we added two sub-routes to the `hello` route. One at `/hello/world` and another one at `hello/mum` with different endpoint functions. We also added an endpoint at `/hello`. This gives an idea what it will be like to build up more complex routing trees

When you have a complex api this also allows you to define different pieces of your route tree in separate functions.

```rust,edition2018,no_run
#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut server = tide::new();

    set_v1_routes(server.at("/api/v1"));
    set_v2_routes(server.at("/api/v2"));

    server.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn set_v1_routes(route: Route) {
    route.at("version").get(|_| async { Ok("Version one") });
}

fn set_v2_routes(route: Route) {
    route.at("version").get(|_| async { Ok("Version two") });
}
```
This example shows for example an API that exposes two different versions. The routes for each version are defined in a separate function.

## Wildcards
There are two wildcard characters we can use `:` and `*`. We already met the `*` wildcard. We used it in the first couple of endpoint examples.
Both wildcard characters will match route segments. Segments are the pieces of a route that are separated with slashes. `:` will match exactly one segment while '*' will match one or more segments.

`"/foo/*/baz"` for example will match against `"/foo/bar/baz"` or `"/foo/bar/qux/baz"`

"foo/:/baz" will match "/foo/bar/baz" but not "/foo/bar/qux/baz", the latter has two segments between foo and baz, while `:` only matches single segments.

### Naming wildcards
It is also possible to name wildcards. This allows you to query the specific strings the wildcard matched on. For example `"/:bar/*baz"` 
will match the string `"/one/two/three"`. You can then query which wildcards matched which parts of the string. In this case `bar` matched `one` while `baz` matched `two/three`. We'll see how you can use this to parse parameters from urls in the next chapter.

### Wildcard precedence
When using wildcards it is possible to define multiple different routes that match the same path.

The routes `"/some/*"` and `"/some/specific/*"` will both match the path `"/some/specific/route"` for example. In many web-frameworks the order in which the routes are defined will determine which route will match. Tide will match the most specific route that matches. In the example it the `"/some/specific/*"` route will match the path.
