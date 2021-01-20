# Request and Response

In the previous chapter we saw how endpoints are simply functions that take a `Request` and return a `Response`, or more accurately a `Result` enum with a type that can be turned into a `Response`
```rust,edition2018,no_run
async fn endpoint(request: tide::Request) -> tide::Result<impl Into<Response>>
```
The `Request` object contains all the information from the HTTP request that was received by the server. The URL from the request, HTTP headers, cookies and query string parameters can all be found in the `Request`.
Additionally the `Request` object in Tide is used to pass information about the application state and the request state into the endpoint. We will look into this in the next chapter about `State`.

The `Response` struct in turn allows us to craft a complete HTTP response. It contains the `Response` body, but also a set of HTTP headers and a response code. While the `Response` struct can be created, accessed and modified directly, it can be convenient to create a `Response` through the Tide `ResponseBuilder`.
