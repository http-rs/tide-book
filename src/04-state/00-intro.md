# State

Tide allows us to use two types of state. The `Server` state is instantiated when the application is started. It can be used to maintain the application state and is available in all Middleware and Endpoints. This is the ideal place to keep database connection pools, application configuration, cached data or session stores. The `Server` state is passed to all middleware and endpoint calls, and those might happen on different threads so there are some restrictions to what types can be used, expressed in a couple of trait bounds.

Tide also provides `Request` state. As the name implies this state is unique for each `Request` and is lost once a request is handled. Why this type of state might be useful will become clear in the next chapter about `Middleware`
`Request` state is available as a type-map on the `Request` struct. Variables can be stored in the `Request` and can be retrieved by their type. 

Multiple pieces of `Request` state can be stored in a request, as long as they have different types. The application state is always one instance of a type. Of course this type can have many fields to store as much application state as you need.
