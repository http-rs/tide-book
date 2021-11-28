# Request
The Tide `Request` struct is the input to your endpoint handler function. It contains all the data from the HTTP request but it is also used by Tide to pass in the application and request `State`. We will look at this in more detail in the next chapter. For now it is enough to know that the `State` generic type parameter of the `Request<State>` type you will see everywhere is the application state.

## Request body
The `Request` provides a set of methods to access the `Request` body. `body_string`, `body_bytes` and `body_json` allow you to read the request body either as a string, as binary data or parse it as json data.
There are a couple of things to keep in mind here. First of all, because a request body can be a sizable piece of data, these methods work asynchronously and can only be called once.

The `body_json` is generic over its return type. Json data can be parsed into any type that implements (or derives) `serde::Deserialize`.

## Accessing Url parameters
In the previous chapter we showed how to use wildcards to match routes. These wildcards can be used to pass parameters into an endpoint as part of the url. To do this you first need to define a route with one or more named wildcards like this;
```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:url-params-route}}
```
This route defines the *some* and *parameters* wildcards. These can then be retrieved from the `Request` using the `Request::param` method like this;

```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:url-params-handler}}
```

## Query parameters
Another way of passing parameters in an HTTP request is in the query string. The query string follows the url and consists of a set of key and value pairs. A question mark signals the start of the query string. The key-value pairs are separated by `&` signs and the keys and values are separated by an equils sign.
`http://www.example.com/query?value1=my_value&value2=32`

Tide allows you to parse the keys and values of the query string in your URL into a structure, it uses the `serde-qs` crate for this. To do this you first need to define a struct to receive the values from the query string;
```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:query-string-struct}}
```
Note that this struct derives `serde::Deserialize` to enable parsing query strings into it.

You can then use the `query()` method on the request to parse the query string from the URL into this struct like in the following endpoint;
```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:query-string-struct-parse}}
```

One trick you can use when you don't want to define a whole new type to receive your query string values, Or when you don't know what keys will be present in your query string, is to parse your query-string into a `HashMap<String, String>` like this;
```rust,ignore
{{#include ../../examples/ch03-request-response/src/main.rs:query-string-hashmap}}
```

## HTTP headers
