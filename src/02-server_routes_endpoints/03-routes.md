# Defining and composing routes

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

```rust,ignore
server.at("/hello").at("world").get(|_| async { Ok("Hello, world!") });
```
This will give you the same result as:

```rust,ignore
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

### Named wildcards

It is also possible to name wildcards. This allows you to query the specific strings the wildcard matched on. For example `"/:bar/*baz"` 
will match the string `"/one/two/three"`. You can then query which wildcards matched which parts of the string. In this case `bar` matched `one` while `baz` matched `two/three`. We'll see how you can use this to parse parameters from urls in the next chapter.

### Wildcard precedence

When using wildcards it is possible to define multiple different routes that match the same path.

The routes `"/some/*"` and `"/some/specific/*"` will both match the path `"/some/specific/route"` for example. In many web-frameworks the order in which the routes are defined will determine which route will match. Tide will match the most specific route that matches. In the given example the `"/some/specific/*"` route will match the path.
