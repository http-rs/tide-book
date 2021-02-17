# The server, Routes and Endpoints

The central part of a Tide application is the `Server` struct. A Tide application is started by creating a
`Server` and configuring it with `Route`s to `Endpoint`s.
When a `Server` is started it will handle incoming `Request`s by matching their URLs with Routes. Requests that match a route are then  dispatched to the corresponding `Endpoint`.