# Example

Create an HTTP server that receives a JSON body, validates it, and responds
with a confirmation message.

```rust
use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
```

```sh
$ curl localhost:8000/orders/shoes -d '{ "name": "Chashu", "legs": 4 }'
```
Hello, Chashu! I've put in an order for 4 shoes

```sh
$ curl localhost:8000/orders/shoes -d '{ "name": "Mary Millipede", "legs": 750 }'
```
number too large to fit in target type

