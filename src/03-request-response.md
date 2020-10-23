# Request and Response

We already described how endpoints are essentially functions that take a `Request` and return a `Response`. The `Request` object contains all information about the HTTP request being made, information like the URL that is requested, HTTP headers, cookies, query string parameters can all be found in the `Request` struct.

The `Request` object in Tide is a bit more than just an HTTP request. It is also used to pass information about the application state and the request state into the endpoint. We will look into this in the next chapter about `State`.

The `Response` struct in turn allows us to craft a complete HTTP response. It contains the `Response` body, but also a set of HTTP headers and a response code. While all this can be accessed directly through the `Response` type. It can be convenient to create a `Response` through the Tide `ResponseBuilder`.

== Request ==


== Response and ResponseBuilder ==

