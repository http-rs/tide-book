# Request and Response

In the previous chapter we saw how endpoints are simply functions that take a `Request` and return a `Response`, or more accurately they return a `Result` of something that implements `Into<Response>`.
```rust,edition2018,no_run
async fn endpoint(request: tide::Request) -> tide::Result<impl Into<Response>>
```
The `Request` struct contains the parsed HTTP request; the URL, HTTP headers, cookies and query string parameters. Additionally the `Request` object in Tide is used to pass information about the application state and the request state to the endpoint. We will look into this in the next chapter about how Tide manages `State`.

The `Response` struct in turn allows us to craft a complete HTTP response. It contains the `Response` body, but also a set of HTTP headers and a response code. While the `Response` struct can be created, accessed and modified directly, it can be convenient to create a `Response` through the Tide `ResponseBuilder`.
