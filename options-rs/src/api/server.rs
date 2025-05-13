use axum::{
    routing::get,
    Router,
    extract::Query,
    response::Html,
    serve::{Serve},
};
use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::State;
use tokio::sync::oneshot;
// Your existing functions here...

// Create a state struct to hold the sender
struct AppState {
    code_sender: Option<oneshot::Sender<String>>,
}

pub async fn run_redirect_server(code_sender: oneshot::Sender<String>) {
    // Create shared state with the sender
    // let state = Arc::new(AppState {
    //     code_sender: Some(code_sender),
    // });

    // build application with a route
    let app = Router::new().route("/callback", get(handle_callback));

    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

// async fn handle_callback(State(state): State<Arc<AppState>>, (params): Query<HashMap<String, String>>) -> Html<String> {
async fn handle_callback((params): Query<HashMap<String, String>>) -> Html<String> {
    if let Some(code) = params.get("code") {
        // Send the code back to the main process
        println!("Received authorization code: {}", code);

        // Take ownership of the sender (replacing it with None)
        // if let Err(e) = state.code_sender.as_ref().expect("Couldnt fine code sender").send(code.to_string()) {
        //     println!("Failed to send code: {}", e);
        // }

        Html(format!(
            "<html><body><h1>Authorization Successful!</h1><p>You can close this window now.</p><p>Code: {}</p></body></html>",
            code
        ))
    } else {
        Html("<html><body><h1>Error: No authorization code received</h1></body></html>".to_string())
    }
}