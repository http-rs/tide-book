## Request
The Tide `Request` struct is the input to your endpoint handler function. It contains all the data from the HTTP request but it is also used by Tide to pass in the application and request `State`. We will look at this in more detail in the next chapter. For now it is enough to know that the `State` generic type parameter of the `Request<State>` type you will see everywhere is the application state. In most simple examples we will not use this state and you will see `Request<()>`.

### Request body

### Accessing Url parameters
In the last chapter where we talked about matching Url routes and specifically in the paragraph about wildcards we already mentioned Url-parameters.

From any route with named wildcards like this;

```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:url-params-route}}
```

Any value that was used to match the a wildcard can be retrieved using the `request.param` method;

```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:url-params-handler}}
```

### The query string and query parameters

### HTTP request headers