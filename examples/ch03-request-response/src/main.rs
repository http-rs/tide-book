// ANCHOR: example
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/:some/:parameters").get(url_params);

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

// ANCHOR END: example
