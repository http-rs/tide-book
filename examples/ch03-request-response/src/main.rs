use serde::Deserialize;
use std::collections::HashMap;
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::new();

    // ANCHOR: url-params-route
    app.at("/url_params/:some/:parameters").get(url_params);
    // ANCHOR_END: url-params-route

    app.at("/query_params").get(query_params);
    app.at("/simple_query").get(simple_query);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

// ANCHOR: url-params-handler
async fn url_params(request: Request<()>) -> tide::Result {
    Ok(format!(
        "Hello, the url contained {} and {}",
        request.param("some").unwrap(),
        request.param("parameters").unwrap()
    )
    .into())
}
// ANCHOR_END: url-params-handler

// ANCHOR: query-string-struct
#[derive(Deserialize)]
struct Query {
    pub parameter1: String,
    pub parameter2: i32,
}
// ANCHOR_END: query-string-struct

// ANCHOR: query-string-struct-parse
async fn query_params(request: Request<()>) -> tide::Result {
    let query: Query = request.query()?;

    Ok(format!(
        "Hello, the query parameters were {} and {}",
        query.parameter1, query.parameter2,
    )
    .into())
}
// ANCHOR_END: query-string-struct-parse

// ANCHOR: query-string-hashmap
async fn simple_query(request: Request<()>) -> tide::Result {
    let query: HashMap<String, String> = request.query()?;

    Ok(format!(
        "Hello, the query parameters were {} and {}",
        query["parameter1"], query["parameter2"],
    )
    .into())
}
// ANCHOR_END: query-string-hashmap
