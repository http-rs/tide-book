# Request
The Tide `Request` struct is the input to your endpoint handler functions. It contains all the data from the HTTP request but it is also used by Tide to pass in the application and request `State`. We will look at this in more detail in the next chapter. For now it is enough to know that the `State` generic type parameter of the `Request<State>` type you will see everywhere is the application state.

## Request body
The `Request` provides a set of methods to access the `Request` body. `body_string`, `body_bytes` and `body_json` allow you to read the request body either as a string, as binary data or parse it as json data.
There are a couple of things to keep in mind here. First of all is that, because a request body can be a sizable piece of data, these methods work asynchronously and can only be called once.
The `body_json` is generic over its return type. Json data can be parsed into any type that implements (or derives) `serde::Deserialize`.

## Query parameters


## Accessing Url parameters
In the last chapter where we talked about matching Url routes and specifically in the paragraph about wildcards we already mentioned Url-parameters.

From any route with named wildcards like this;

```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:url-params-route}}
```

Any value that was used to match the a wildcard can be retrieved using the `request.param` method;

```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:url-params-handler}}
```


## HTTP headers
