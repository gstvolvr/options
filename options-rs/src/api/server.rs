use axum::{
    routing::get,
    Router,
    extract::Query,
    response::Html,
};
use std::collections::HashMap;
use tokio::sync::oneshot;

// Create a state struct to hold the sender
struct AppState {
    code_sender: Option<oneshot::Sender<String>>,
}

pub async fn run_redirect_server(code_sender: oneshot::Sender<String>) {
    // build application with a route
    let app = Router::new().route("/callback", get(handle_callback));

    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn handle_callback((params): Query<HashMap<String, String>>) -> Html<String> {
    if let Some(code) = params.get("code") {
        // Send the code back to the main process
        println!("Received authorization code: {}", code);

        Html(format!(
            "<html><body><h1>Authorization Successful!</h1><p>You can close this window now.</p><p>Code: {}</p></body></html>",
            code
        ))
    } else {
        Html("<html><body><h1>Error: No authorization code received</h1></body></html>".to_string())
    }
}