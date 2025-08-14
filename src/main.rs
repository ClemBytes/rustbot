use std::sync::{Arc, Mutex};

use askama::Template;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use axum_cookie::prelude::*;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct GridState {
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
        .with_state(grid_state)
        .layer(CookieLayer::default());

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(state): State<GridState>, cookie: CookieManager) -> impl IntoResponse {
    // Get grid size
    let grid_max = state.grid_max_coordinates.lock().unwrap();

    // Initialize coordinates:
    let mut i_coord = 0;
    let mut j_coord = 0;

    // Retrieve cookies if already existing
    if let Some(i_cookie) = cookie.get("i") {
        i_coord = i_cookie.value().parse().unwrap();
    }
    if let Some(j_cookie) = cookie.get("j") {
        j_coord = j_cookie.value().parse().unwrap();
    }

    // Add cookies
    cookie.add(Cookie::new("i", format!("{i_coord}")));
    cookie.add(Cookie::new("j", format!("{j_coord}")));

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i: grid_max.0,
        grid_max_j: grid_max.1,
    };
    Html(html.render().unwrap())
}

async fn reset(State(state): State<GridState>, cookie: CookieManager) -> impl IntoResponse {
    // Max grid size
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    let (i_max, j_max) = *grid_max;

    // Update rustbot coordinates
    let i_coord = 0;
    let j_coord = 0;

    // Add cookies
    cookie.add(Cookie::new("i", format!("{i_coord}")));
    cookie.add(Cookie::new("j", format!("{j_coord}")));

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i: i_max,
        grid_max_j: j_max,
    };
    Html(html.render().unwrap())
}

async fn down(State(state): State<GridState>, cookie: CookieManager) -> impl IntoResponse {
    // Max grid size
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    let (i_max, j_max) = *grid_max;

    // Initialize coordinates:
    let mut i_coord = 0;
    let mut j_coord = 0;

    // Retrieve cookies if already existing
    if let Some(i_cookie) = cookie.get("i") {
        i_coord = i_cookie.value().parse().unwrap();
    }
    if let Some(j_cookie) = cookie.get("j") {
        j_coord = j_cookie.value().parse().unwrap();
    }

    // Update rustbot coordinates
    if i_coord == i_max - 1 {
        i_coord = 0;
    } else {
        i_coord += 1;
    }

    // Add cookies
    cookie.add(Cookie::new("i", format!("{i_coord}")));
    cookie.add(Cookie::new("j", format!("{j_coord}")));

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i: i_max,
        grid_max_j: j_max,
    };
    Html(html.render().unwrap())
}

async fn up(State(state): State<GridState>, cookie: CookieManager) -> impl IntoResponse {
    // Max grid size
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    let (i_max, j_max) = *grid_max;

    // Initialize coordinates:
    let mut i_coord = 0;
    let mut j_coord = 0;

    // Retrieve cookies if already existing
    if let Some(i_cookie) = cookie.get("i") {
        i_coord = i_cookie.value().parse().unwrap();
    }
    if let Some(j_cookie) = cookie.get("j") {
        j_coord = j_cookie.value().parse().unwrap();
    }

    // Update rustbot coordinates
    if i_coord == 0 {
        i_coord = i_max - 1;
    } else {
        i_coord -= 1;
    }

    // Add cookies
    cookie.add(Cookie::new("i", format!("{i_coord}")));
    cookie.add(Cookie::new("j", format!("{j_coord}")));

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i: i_max,
        grid_max_j: j_max,
    };
    Html(html.render().unwrap())
}

async fn right(State(state): State<GridState>, cookie: CookieManager) -> impl IntoResponse {
    // Max grid size
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    let (i_max, j_max) = *grid_max;

    // Initialize coordinates:
    let mut i_coord = 0;
    let mut j_coord = 0;

    // Retrieve cookies if already existing
    if let Some(i_cookie) = cookie.get("i") {
        i_coord = i_cookie.value().parse().unwrap();
    }
    if let Some(j_cookie) = cookie.get("j") {
        j_coord = j_cookie.value().parse().unwrap();
    }

    // Update rustbot coordinates
    if j_coord == j_max - 1 {
        j_coord = 0;
    } else {
        j_coord += 1;
    }

    // Add cookies
    cookie.add(Cookie::new("i", format!("{i_coord}")));
    cookie.add(Cookie::new("j", format!("{j_coord}")));

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i: i_max,
        grid_max_j: j_max,
    };
    Html(html.render().unwrap())
}

async fn left(State(state): State<GridState>, cookie: CookieManager) -> impl IntoResponse {
    // Max grid size
    let grid_max = state.grid_max_coordinates.lock().unwrap();
    let (i_max, j_max) = *grid_max;

    // Initialize coordinates:
    let mut i_coord = 0;
    let mut j_coord = 0;

    // Retrieve cookies if already existing
    if let Some(i_cookie) = cookie.get("i") {
        i_coord = i_cookie.value().parse().unwrap();
    }
    if let Some(j_cookie) = cookie.get("j") {
        j_coord = j_cookie.value().parse().unwrap();
    }

    // Update rustbot coordinates
    if j_coord == 0 {
        j_coord = j_max - 1;
    } else {
        j_coord -= 1;
    }

    // Add cookies
    cookie.add(Cookie::new("i", format!("{i_coord}")));
    cookie.add(Cookie::new("j", format!("{j_coord}")));

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i: i_max,
        grid_max_j: j_max,
    };
    Html(html.render().unwrap())
}
