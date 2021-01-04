# Request and Response

In the previous chapter we saw how endpoints are functions that take a `Request` and return a `Response`, or more accurately a `Result` enum with a type that can be turned into a `Response`
```rust,edition2018,no_run
async fn endpoint(request: tide::Request) -> tide::Result<impl Into<Response>>
```
The `Request` object contains all the information from the HTTP request that was received by the server. The URL from the request, HTTP headers, cookies and query string parameters can all be found in the `Request`.
Additionally the `Request` object in Tide is used to pass information about the application state and the request state into the endpoint. We will look into this in the next chapter about `State`.

The `Response` struct in turn allows us to craft a complete HTTP response. It contains the `Response` body, but also a set of HTTP headers and a response code. While the `Response` struct can be created, accessed and modified directly, it can be convenient to create a `Response` through the Tide `ResponseBuilder`.

## Request
The Tide `Request` struct is the input to your endpoint handler function. It contains all the data from the HTTP request but it is also used by Tide to pass in the application and request `State`. We will look at this in more detail in the next chapter. For now it is enough to know that the `State` generic type parameter of the `Request<State>` type you will see everywhere is the application state. In most simple examples we will not use this state and you will see `Request<()>`.

### Request body

### Accessing Url parameters
In the last chapter where we talked about matching Url routes and specifically in the paragraph about wildcards we already mentioned Url-parameters.

From any route with named wildcards like this;

```rust,ignore
{{#include ../examples/ch03-request-response/src/main.rs:url-params-route}}
```

Any value that was used to match the a wildcard can be retrieved using the `request.param` method;

```rust,ignore
{{#include ../examples/ch03-request-response/src/main.rs:url-params-handler}}
```

### The query string and query parameters

### HTTP request headers

## Response and the ResponseBuilder

