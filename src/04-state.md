# Server and request state

## Server State
Until now endpoints were simple stateless functions that processed a request into a response. But for any serious application we would need to be able to maintain some state somewhere. In a real life application we would need to have a place to store things like sessions, database connection pools, configuration etc. And we would rather not use global variables to do this.

Tide gives us Server state to do just this. If you look at the definition of the Server struct you see that it has one generic type parameter called `State`. Because we've been creating `Server`s with the `Server::new` factory method so far we have been using `Server<()>`. But we can pass in our own state struct or enum when creating the Server. This will then be passed into all endpoint handlers through the `Request`.

### Set up state for an application

### Accessing state