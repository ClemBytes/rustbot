use askama::Template;
use axum::{
    Router,
    extract::{Form, Path},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use axum_cookie::prelude::*;
use serde::Deserialize;
use tower_http::services::ServeDir;

const DEFAULT_MAX_I: u32 = 5;
const DEFAULT_MAX_J: u32 = 5;

#[derive(Deserialize)]
struct MaxGridSizes {
    change_max_i: u32,
    change_max_j: u32,
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
    // Build app with different routes
    let app = Router::new()
        .route("/", get(root))
        .route("/reset", get(reset).post(reset))
        .route("/right", get(right).post(right))
        .route("/left", get(left).post(left))
        .route("/down", get(down).post(down))
        .route("/up", get(up).post(up))
        .route("/coords/{i}/{j}", get(teleport).post(teleport))
        .route("/change-max", post(change_max))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CookieLayer::default());

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_grid_size(cookie: &CookieManager) -> (u32, u32) {
    let mut grid_max_i = DEFAULT_MAX_I;
    let mut grid_max_j = DEFAULT_MAX_J;
    if let Some(max_i_cookie) = cookie.get("max-i") {
        grid_max_i = max_i_cookie.value().parse().unwrap();
    }
    if let Some(max_j_cookie) = cookie.get("max-j") {
        grid_max_j = max_j_cookie.value().parse().unwrap();
    }
    (grid_max_i, grid_max_j)
}

fn get_rustbot_coordinates(cookie: &CookieManager) -> (u32, u32) {
    let mut i_coord = 0;
    let mut j_coord = 0;
    if let Some(i_cookie) = cookie.get("i") {
        i_coord = i_cookie.value().parse().unwrap();
    }
    if let Some(j_cookie) = cookie.get("j") {
        j_coord = j_cookie.value().parse().unwrap();
    }
    (i_coord, j_coord)
}

async fn root(cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);
    let (i_coord, j_coord) = get_rustbot_coordinates(&cookie);

    // Add cookies
    let mut cookie_i = Cookie::new("i", format!("{i_coord}"));
    cookie_i.set_path("/");
    cookie.add(cookie_i);
    let mut cookie_j = Cookie::new("j", format!("{j_coord}"));
    cookie_j.set_path("/");
    cookie.add(cookie_j);
    let mut cookie_max_i = Cookie::new("max-i", format!("{grid_max_i}"));
    cookie_max_i.set_path("/");
    cookie.add(cookie_max_i);
    let mut cookie_max_j = Cookie::new("max-j", format!("{grid_max_j}"));
    cookie_max_j.set_path("/");
    cookie.add(cookie_max_j);

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn reset(cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);

    // Update rustbot coordinates
    let i_coord = 0;
    let j_coord = 0;

    // Add cookies
    let mut cookie_i = Cookie::new("i", format!("{i_coord}"));
    cookie_i.set_path("/");
    cookie.add(cookie_i);
    let mut cookie_j = Cookie::new("j", format!("{j_coord}"));
    cookie_j.set_path("/");
    cookie.add(cookie_j);
    let mut cookie_max_i = Cookie::new("max-i", format!("{grid_max_i}"));
    cookie_max_i.set_path("/");
    cookie.add(cookie_max_i);
    let mut cookie_max_j = Cookie::new("max-j", format!("{grid_max_j}"));
    cookie_max_j.set_path("/");
    cookie.add(cookie_max_j);

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn down(cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);
    let (mut i_coord, j_coord) = get_rustbot_coordinates(&cookie);

    // Update rustbot coordinates
    if i_coord == grid_max_i - 1 {
        i_coord = 0;
    } else {
        i_coord += 1;
    }

    // Add cookies
    let mut cookie_i = Cookie::new("i", format!("{i_coord}"));
    cookie_i.set_path("/");
    cookie.add(cookie_i);
    let mut cookie_j = Cookie::new("j", format!("{j_coord}"));
    cookie_j.set_path("/");
    cookie.add(cookie_j);
    let mut cookie_max_i = Cookie::new("max-i", format!("{grid_max_i}"));
    cookie_max_i.set_path("/");
    cookie.add(cookie_max_i);
    let mut cookie_max_j = Cookie::new("max-j", format!("{grid_max_j}"));
    cookie_max_j.set_path("/");
    cookie.add(cookie_max_j);

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn up(cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);
    let (mut i_coord, j_coord) = get_rustbot_coordinates(&cookie);

    // Update rustbot coordinates
    if i_coord == 0 {
        i_coord = grid_max_i - 1;
    } else {
        i_coord -= 1;
    }

    // Add cookies
    let mut cookie_i = Cookie::new("i", format!("{i_coord}"));
    cookie_i.set_path("/");
    cookie.add(cookie_i);
    let mut cookie_j = Cookie::new("j", format!("{j_coord}"));
    cookie_j.set_path("/");
    cookie.add(cookie_j);
    let mut cookie_max_i = Cookie::new("max-i", format!("{grid_max_i}"));
    cookie_max_i.set_path("/");
    cookie.add(cookie_max_i);
    let mut cookie_max_j = Cookie::new("max-j", format!("{grid_max_j}"));
    cookie_max_j.set_path("/");
    cookie.add(cookie_max_j);

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn right(cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);
    let (i_coord, mut j_coord) = get_rustbot_coordinates(&cookie);

    // Update rustbot coordinates
    if j_coord == grid_max_j - 1 {
        j_coord = 0;
    } else {
        j_coord += 1;
    }

    // Add cookies
    let mut cookie_i = Cookie::new("i", format!("{i_coord}"));
    cookie_i.set_path("/");
    cookie.add(cookie_i);
    let mut cookie_j = Cookie::new("j", format!("{j_coord}"));
    cookie_j.set_path("/");
    cookie.add(cookie_j);
    let mut cookie_max_i = Cookie::new("max-i", format!("{grid_max_i}"));
    cookie_max_i.set_path("/");
    cookie.add(cookie_max_i);
    let mut cookie_max_j = Cookie::new("max-j", format!("{grid_max_j}"));
    cookie_max_j.set_path("/");
    cookie.add(cookie_max_j);

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn left(cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);
    let (i_coord, mut j_coord) = get_rustbot_coordinates(&cookie);

    // Update rustbot coordinates
    if j_coord == 0 {
        j_coord = grid_max_j - 1;
    } else {
        j_coord -= 1;
    }

    // Add cookies
    let mut cookie_i = Cookie::new("i", format!("{i_coord}"));
    cookie_i.set_path("/");
    cookie.add(cookie_i);
    let mut cookie_j = Cookie::new("j", format!("{j_coord}"));
    cookie_j.set_path("/");
    cookie.add(cookie_j);
    let mut cookie_max_i = Cookie::new("max-i", format!("{grid_max_i}"));
    cookie_max_i.set_path("/");
    cookie.add(cookie_max_i);
    let mut cookie_max_j = Cookie::new("max-j", format!("{grid_max_j}"));
    cookie_max_j.set_path("/");
    cookie.add(cookie_max_j);

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn teleport(
    cookie: CookieManager,
    Path((i_teleport, j_teleport)): Path<(u32, u32)>,
) -> impl IntoResponse {
    // Initialize coordinates:
    let i_coord = i_teleport;
    let j_coord = j_teleport;

    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);

    // Add cookies
    // Need set_path("/") to avoid duplicating the cookies for different URLs
    let mut cookie_i = Cookie::new("i", format!("{i_coord}"));
    cookie_i.set_path("/");
    cookie.add(cookie_i);
    let mut cookie_j = Cookie::new("j", format!("{j_coord}"));
    cookie_j.set_path("/");
    cookie.add(cookie_j);
    let mut cookie_max_i = Cookie::new("max-i", format!("{grid_max_i}"));
    cookie_max_i.set_path("/");
    cookie.add(cookie_max_i);
    let mut cookie_max_j = Cookie::new("max-j", format!("{grid_max_j}"));
    cookie_max_j.set_path("/");
    cookie.add(cookie_max_j);

    // Create html response
    let html = MainTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn change_max(
    cookie: CookieManager,
    Form(max_grid_sizes): Form<MaxGridSizes>,
) -> impl IntoResponse {
    let grid_max_i = max_grid_sizes.change_max_i;
    let grid_max_j = max_grid_sizes.change_max_j;
    // Add cookies
    // Need set_path("/") to avoid duplicating the cookies for different URLs
    let mut cookie_i = Cookie::new("i", "0");
    cookie_i.set_path("/");
    cookie.add(cookie_i);
    let mut cookie_j = Cookie::new("j", "0");
    cookie_j.set_path("/");
    cookie.add(cookie_j);
    let mut cookie_max_i = Cookie::new("max-i", format!("{grid_max_i}"));
    cookie_max_i.set_path("/");
    cookie.add(cookie_max_i);
    let mut cookie_max_j = Cookie::new("max-j", format!("{grid_max_j}"));
    cookie_max_j.set_path("/");
    cookie.add(cookie_max_j);

    // Create html response
    let html = MainTemplate {
        rustbot_i: 0,
        rustbot_j: 0,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}
