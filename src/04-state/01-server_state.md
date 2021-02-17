# Server State
Until now endpoints were simple stateless functions that process requests into a responses. But for any serious application we need to be able to maintain state somewhere. In a real life application we need to have a place to store things like sessions, database connection pools, configuration etc. And we would rather not use global variables for this.

Tide gives us Server state to do just this. If you look at the definition of the Server struct you see that it has one generic type parameter called `State`. We've been using `Server::new` to construct our `Server` instances, which returns a server without state; `Server<()>`. But the `Server` type also has a `Server::with_state` constructor where you can pass in our own state instance when creating the Server. This State will then be passed to all endpoint handlers through the `Request` parameter.

## Setup state for an application
To set up our application state we first need to have a type to store the application data that will be shared between requests.
```rust,ignore
{{#include ../../examples/ch04-server-state/src/main.rs:appstate}}
```
In this example we will share a simple counter. We use an Arc<AtomicU32> to make sure we can safely share this even when simultanious requests come in.

To set the state in the `tide::server` we need to use a different constructor than the `server::new()` we used previously. We can use `server::with_state(...)` to set up a server with state.
```rust,ignore
{{#include ../../examples/ch04-server-state/src/main.rs:start_with_state}}
```

## Accessing state
The state can then be accessed using the `state` method on your `Request` inside your endpoints;
```rust,ignore
{{#include ../../examples/ch04-server-state/src/main.rs:read_state_request}}
```

```rust,ignore
{{#include ../../examples/ch04-server-state/src/main.rs:update_state_request}}
```

## Server state limitations
As you can see in the previous example the `State` object does have some limitations. First of all Tide sets some trait bounds. The `State` needs to be `Clone`, `Send` and `Sync`. This is because it will be passed into all your endpoints, these might be running concurrently on different threads than where you created the `State`
When accessing `State` in your endpoints it's only available as a non-mutable reference.

To get around these limitations for our counter we used an AtomicU32 that provides the internal mutability for our counter and we wrapped it in an `Arc` to be able to copy it around.

In practice this is often not that much of a problem. Many database-access libraries for example already provide connection pools components that are written to be able to be passed around inside an application like this. <TODO>point at an sqlx examle</TODO>