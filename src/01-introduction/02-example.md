# Example

Create an HTTP server that receives a JSON body, validates it, and responds
with a confirmation message.

```rust
{{#include ../../examples/ch01-02-example/src/main.rs:example}}
```

```sh
$ curl localhost:8000/orders/shoes -d '{ "name": "Chashu", "legs": 4 }'
```
Hello, Chashu! I've put in an order for 4 shoes

```sh
$ curl localhost:8000/orders/shoes -d '{ "name": "Mary Millipede", "legs": 750 }'
```
number too large to fit in target type

