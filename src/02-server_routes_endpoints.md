# The server, Routes and Endpoints

The central part of a Tide application is the `Server` struct. A Tide application is started by creating a
`Server`, configuring it with `State`, `Route`s, `Endpoint`s and `Middleware`. And then starting it.
When a `Server` is started it will handle incoming `Request`s by matching their URLs with Routes and dispatching every `Request` to the right `Endpoint`

== Route ==


== Endpoint ==
