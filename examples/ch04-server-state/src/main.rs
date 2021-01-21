use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};
use tide::Request;

// ANCHOR: appstate
#[derive(Clone)]
struct AppState {
    pub datastore: Arc<AtomicU32>,
}
// ANCHOR_END: appstate

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    // ANCHOR: start_with_state
    let mut app = tide::with_state(AppState {
        datastore: Arc::new(AtomicU32::new(0)),
    });
    // ANCHOR_END: start_with_state

    app.at("/update_state").get(update_state);
    app.at("/read_state").get(read_state);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

// ANCHOR: update_state_request
async fn update_state(request: Request<AppState>) -> tide::Result {
    request.state().datastore.fetch_add(1, Ordering::SeqCst);

    Ok("datastore updated".into())
}
// ANCHOR_END: update_state_request

// ANCHOR: read_state_request
async fn read_state(request: Request<AppState>) -> tide::Result {
    Ok(format!(
        "datastore has been updated {} times",
        request.state().datastore.load(Ordering::SeqCst)
    )
    .into())
}
// ANCHOR_END: read_state_request
