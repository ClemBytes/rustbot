use std::sync::{Arc, Mutex};

use axum::{
    extract::State, routing::get, Router
};

#[derive(Clone)]
struct Coordinates {
    coordinates: Arc<Mutex<(i32, i32)>>,
}

#[tokio::main]
async fn main() {
    let coords = Coordinates {
        coordinates: Arc::new(Mutex::new((0, 0))),
    };

    // Build app with different routes
    let app = Router::new()
        .route("/", get(root))
        .route("/reset", get(reset))
        .route("/right", get(right))
        .route("/left", get(left))
        .route("/down", get(down))
        .route("/up", get(up))
        .with_state(coords);

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(state): State<Coordinates>) -> String {
    let coords = state.coordinates.lock().unwrap();
    format!("Rustbot coordinates: ({}, {})", coords.0, coords.1)
}

async fn reset(State(state): State<Coordinates>) -> String {
    let mut coords = state.coordinates.lock().unwrap();
    coords.0 = 0;
    coords.1 = 0;
    format!("Rustbot coordinates: ({}, {})", coords.0, coords.1)
}

async fn right(State(state): State<Coordinates>) -> String {
    let mut coords = state.coordinates.lock().unwrap();
    coords.1 += 1;
    format!("Rustbot coordinates: ({}, {})", coords.0, coords.1)
}

async fn left(State(state): State<Coordinates>) -> String {
    let mut coords = state.coordinates.lock().unwrap();
    coords.1 -= 1;
    format!("Rustbot coordinates: ({}, {})", coords.0, coords.1)
}

async fn down(State(state): State<Coordinates>) -> String {
    let mut coords = state.coordinates.lock().unwrap();
    coords.0 += 1;
    format!("Rustbot coordinates: ({}, {})", coords.0, coords.1)
}

async fn up(State(state): State<Coordinates>) -> String {
    let mut coords = state.coordinates.lock().unwrap();
    coords.0 -= 1;
    format!("Rustbot coordinates: ({}, {})", coords.0, coords.1)
}