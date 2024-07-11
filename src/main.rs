pub mod handlers;
pub mod state;

use std::sync::Arc;

use axum::{
    routing::{get, put},
    Router,
};
use state::ServiceState;

#[tokio::main]
async fn main() {
    let state = Arc::new(ServiceState::default());

    // build our application with a single route
    let app = Router::new()
        .route("/:key", get(handlers::get))
        .route("/:key", put(handlers::put))
        .with_state(state);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
