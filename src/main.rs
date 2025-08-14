use std::sync::{Arc, Mutex};

use askama::Template;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct GridState {
    rustbot_coordinates: Arc<Mutex<(u32, u32)>>,
    grid_max_coordinates: Arc<Mutex<(u32, u32)>>,
}

#[derive(Template)]
#[template(path = "template.html")]
struct MainTemplate {
    rustbot_i: u32,
    rustbot_j: u32,
    grid_max_i: u32,
    grid_max_j: u32,
}

#[tokio::main]
async fn main() {
    let grid_state = GridState {
        rustbot_coordinates: Arc::new(Mutex::new((0, 0))),
        grid_max_coordinates: Arc::new(Mutex::new((5, 5))),
    };

    // Build app with different routes
    let app = Router::new()
        .route("/", get(root))
        .route("/reset", get(reset).post(reset))
        .route("/right", get(right).post(right))
        .route("/left", get(left).post(left))
        .route("/down", get(down).post(down))
        .route("/up", get(up).post(up))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(grid_state);

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(
    State(state): State<GridState>,
) -> impl IntoResponse {
    let coords = state.rustbot_coordinates.lock().unwrap();
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    let html = MainTemplate {
        rustbot_i: coords.0,
        rustbot_j: coords.1,
        grid_max_i: grid_max.0,
        grid_max_j: grid_max.1,
    };
    Html(html.render().unwrap())
}

async fn reset(
    State(state): State<GridState>,
) -> impl IntoResponse {
    let mut coords = state.rustbot_coordinates.lock().unwrap();
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    coords.0 = 0;
    coords.1 = 0;
    let html = MainTemplate {
        rustbot_i: coords.0,
        rustbot_j: coords.1,
        grid_max_i: grid_max.0,
        grid_max_j: grid_max.1,
    };
    Html(html.render().unwrap())
}

async fn right(
    State(state): State<GridState>,
) -> impl IntoResponse {
    let mut coords = state.rustbot_coordinates.lock().unwrap();
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    coords.1 += 1;
    let html = MainTemplate {
        rustbot_i: coords.0,
        rustbot_j: coords.1,
        grid_max_i: grid_max.0,
        grid_max_j: grid_max.1,
    };
    Html(html.render().unwrap())
}

async fn left(
    State(state): State<GridState>,
) -> impl IntoResponse {
    let mut coords = state.rustbot_coordinates.lock().unwrap();
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    coords.1 -= 1;
    let html = MainTemplate {
        rustbot_i: coords.0,
        rustbot_j: coords.1,
        grid_max_i: grid_max.0,
        grid_max_j: grid_max.1,
    };
    Html(html.render().unwrap())
}

async fn down(
    State(state): State<GridState>,
) -> impl IntoResponse {
    let mut coords = state.rustbot_coordinates.lock().unwrap();
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    coords.0 += 1;
    let html = MainTemplate {
        rustbot_i: coords.0,
        rustbot_j: coords.1,
        grid_max_i: grid_max.0,
        grid_max_j: grid_max.1,
    };
    Html(html.render().unwrap())
}

async fn up(
    State(state): State<GridState>,
) -> impl IntoResponse {
    let mut coords = state.rustbot_coordinates.lock().unwrap();
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    coords.0 -= 1;
    let html = MainTemplate {
        rustbot_i: coords.0,
        rustbot_j: coords.1,
        grid_max_i: grid_max.0,
        grid_max_j: grid_max.1,
    };
    Html(html.render().unwrap())
}
