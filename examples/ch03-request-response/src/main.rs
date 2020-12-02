// ANCHOR: example
use serde::Deserialize;
use std::collections::HashMap;
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/:some/:parameters").get(url_params);
    app.at("/query_params").get(query_params);
    app.at("/simple_query").get(simple_query);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn url_params(request: Request<()>) -> tide::Result {
    Ok(format!(
        "Hello, the url contained {} and {}",
        request.param("some").unwrap(),
        request.param("parameters").unwrap()
    )
    .into())
}

#[derive(Deserialize)]
struct Query {
    pub parameter1: String,
    pub parameter2: i32,
}

async fn query_params(request: Request<()>) -> tide::Result {
    let query: Query = request.query()?;

    Ok(format!(
        "Hello, the query parameters were {} and {}",
        query.parameter1, query.parameter2,
    )
    .into())
}

async fn simple_query(request: Request<()>) -> tide::Result {
    let query: HashMap<String, String> = request.query()?;

    Ok(format!(
        "Hello, the query parameters were {} and {}",
        query["parameter1"], query["parameter2"],
    )
    .into())
}

// ANCHOR END: example
