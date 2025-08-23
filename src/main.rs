use askama::Template;
use axum::{
    Router,
    extract::{Form, Path},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use axum_cookie::prelude::*;
use regex::Regex;
use serde::Deserialize;
use tower_http::services::ServeDir;

// Default values for grid size
/// Default number of lines in grid
const DEFAULT_MAX_I: u32 = 5;
/// Default number of columns in grid
const DEFAULT_MAX_J: u32 = 5;

// Max values authorized for grid size
/// Maximum authorized number of lines
const MAX_MAX_I: u32 = 20;
/// Maximum authorized number of columns
const MAX_MAX_J: u32 = 20;

/// Struct representing the new grid sizes submitted via a form.
///
/// Used to deserialize the POST request payload from `/change-max`.
#[derive(Deserialize)]
struct MaxGridSizes {
    change_max_i: u32,
    change_max_j: u32,
}

#[derive(Deserialize)]
struct UserCode {
    user_code: String,
}

/// Template context for the root page.
///
/// Passed to Askama to render `template_root.html`.
#[derive(Template)]
#[template(path = "template_root.html")]
struct RootTemplate {}

/// Template context for the play mode page.
///
/// Passed to Askama to render `template_play.html`.
#[derive(Template)]
#[template(path = "template_play.html")]
struct PlayTemplate {
    rustbot_i: u32,
    rustbot_j: u32,
    grid_max_i: u32,
    grid_max_j: u32,
}

/// Template context for the code mode page.
///
/// Passed to Askama to render `template_code.html`.
#[derive(Template)]
#[template(path = "template_code.html")]
struct CodeTemplate {
    rustbot_i: u32,
    rustbot_j: u32,
    grid_max_i: u32,
    grid_max_j: u32,
}

/// Launches the RustBot web server
///
/// # Description
/// This `async` function sets up the Axum application, defining all routes for
/// controlling the bot and serving static files. It then binds a TCP listener
/// on port 3000 and starts serving requests using Hyper.
///
/// # Routes
/// - `/` → `root`: shows current coordinates
/// - `/reset` → `reset`: reset coordinates to (0, 0)
/// - `/right` → `right`: move bot right
/// - `/left` → `left`: move bot left
/// - `/down` → `down`: move bot down
/// - `/up` → `up`: move bot up
/// - `/coords/{i}/{j}` → `teleport`: teleport bot to specific coordinates
/// - `/change-max` → `change_max`: update grid size via form submission
/// - `/static` → serves static files from `static` directory
///
/// # Notes
/// - Uses `CookieLayer` for storing coordinates in cookies.
/// - Listens globally on `0.0.0.0:3000`.
///
/// # Example
/// ```no_run
/// // Simply run the server:
/// cargo run
/// ```
#[tokio::main]
async fn main() {
    // Build app with different routes
    let app = Router::new()
        // Root: main page
        .route("/", get(root))
        // Play mode:
        .route("/play", get(play))
        .route("/reset", get(reset).post(reset))
        .route("/right", get(right).post(right))
        .route("/left", get(left).post(left))
        .route("/down", get(down).post(down))
        .route("/up", get(up).post(up))
        .route("/coords/{i}/{j}", get(teleport).post(teleport))
        .route("/change-max", post(change_max))
        // Code mode:
        .route("/code", get(code))
        .route("/user-code", post(user_code))
        // Static pages
        .nest_service("/static", ServeDir::new("static"))
        .layer(CookieLayer::default());

    // run app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Reads the current grid size from cookies.
///
/// This function checks the cookies `"max-i"` and `"max-j"` to determine
/// the number of rows (`i`) and columns (`j`) of the grid. If the cookies
/// are not present, it returns default values (`DEFAULT_MAX_I` and `DEFAULT_MAX_J`).
/// 
/// If parse fails (for exemple if user changes cookies for absurd values), returns
/// default values. Also forbids to have values set bigger than MAX_MAX_I/J.
///
/// # Parameters
/// - `cookie`: Reference to the `CookieManager` from which to read the cookies.
///
/// # Returns
/// A tuple `(grid_max_i, grid_max_j)` representing the number of rows and columns.
///
/// # Panics
/// This function will panic if the cookie values are present but cannot be parsed
/// into `u32`.
///
/// # Example
/// ```no_run
/// let cookie_manager = CookieManager::new();
/// let (rows, cols) = get_grid_size(&cookie_manager);
/// println!("Grid size: {}x{}", rows, cols);
/// ```
fn get_grid_size(cookie: &CookieManager) -> (u32, u32) {
    let mut grid_max_i = DEFAULT_MAX_I;
    let mut grid_max_j = DEFAULT_MAX_J;
    if let Some(max_i_cookie) = cookie.get("max-i") {
        match max_i_cookie.value().parse() {
            Ok(cookie_max_i) => {
                if cookie_max_i > MAX_MAX_I {
                    grid_max_i = MAX_MAX_I;
                } else {
                    grid_max_i = cookie_max_i;
                }
            },
            Err(_) => grid_max_i = DEFAULT_MAX_I,
        }
    }
    if let Some(max_j_cookie) = cookie.get("max-j") {
        match max_j_cookie.value().parse() {
            Ok(cookie_max_j) => {
                if cookie_max_j > MAX_MAX_J {
                    grid_max_j = MAX_MAX_J;
                } else {
                    grid_max_j = cookie_max_j;
                }
            },
            Err(_) => grid_max_j = DEFAULT_MAX_J,
        }
    }
    (grid_max_i, grid_max_j)
}

/// Retrieves Rustbot's current coordinates from cookies.
///
/// If the cookies `"i"` or `"j"` are not present, defaults to `(0, 0)`.
/// 
/// If parse fails (for exemple if user changes cookies for absurd values), returns
/// to 0. Also forbids to have values set bigger than grid size.
///
/// # Arguments
///
/// * `cookie` - A reference to the `CookieManager` containing the cookies.
///
/// # Returns
///
/// A tuple `(i_coord, j_coord)` representing Rustbot's row and column positions.
fn get_rustbot_coordinates(cookie: &CookieManager) -> (u32, u32) {
    let (grid_size_i, grid_size_j) = get_grid_size(cookie);
    let mut i_coord = 0;
    let mut j_coord = 0;
    if let Some(i_cookie) = cookie.get("i") {
        match i_cookie.value().parse() {
            Ok(cookie_i) => {
                if cookie_i > grid_size_i {
                    i_coord = grid_size_i - 1;
                } else {
                    i_coord = cookie_i;
                }
            },
            Err(_) => i_coord = 0,
        }
    }
    if let Some(j_cookie) = cookie.get("j") {
        match j_cookie.value().parse() {
            Ok(cookie_j) => {
                if cookie_j > grid_size_j {
                    j_coord = grid_size_j - 1;
                } else {
                    j_coord = cookie_j;
                }
            },
            Err(_) => j_coord = 0,
        }
    }
    (i_coord, j_coord)
}

/// Updates Rustbot's coordinates and grid size cookies.
///
/// Sets the cookies `"i"`, `"j"`, `"max-i"`, and `"max-j"` to the provided values,
/// and ensures their path is `/` to make them available site-wide.
///
/// # Arguments
///
/// * `i_coord` - The current row of Rustbot.
/// * `j_coord` - The current column of Rustbot.
/// * `grid_max_i` - Maximum number of rows in the grid.
/// * `grid_max_j` - Maximum number of columns in the grid.
/// * `cookie` - A mutable reference to the `CookieManager` used to store the cookies.
fn update_cookie(
    i_coord: u32,
    j_coord: u32,
    grid_max_i: u32,
    grid_max_j: u32,
    cookie: &mut CookieManager,
) {
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
}

/// Handler for the root path `/`.
///
/// Retrieves Rustbot's coordinates and the grid size from cookies, updates them if necessary,
/// and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template, implementing `IntoResponse`.
async fn root() -> impl IntoResponse {
    // Create html response
    let html = RootTemplate {};
    Html(html.render().unwrap())
}

/// Handler for the play path `/`.
///
/// Retrieves Rustbot's coordinates and the grid size from cookies, updates them if necessary,
/// and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template, implementing `IntoResponse`.
async fn play(mut cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);
    let (i_coord, j_coord) = get_rustbot_coordinates(&cookie);

    // Add cookies
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = PlayTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

/// Handler for resetting Rustbot's coordinates to `(0, 0)`.
///
/// Retrieves the current grid size from cookies, resets the Rustbot coordinates,
/// updates the cookies accordingly, and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template, implementing `IntoResponse`.
///
/// # Panics
///
/// Will panic if rendering the template fails (`unwrap()` on `html.render()`).
async fn reset(mut cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);

    // Update rustbot coordinates
    let i_coord = 0;
    let j_coord = 0;

    // Add cookies
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = PlayTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

/// Handler to move Rustbot **down** by one row in the grid.
///
/// Retrieves the current grid size and Rustbot coordinates from cookies,
/// increments the `i` coordinate (row) by one, wrapping around to 0 if it
/// reaches the maximum, updates the cookies, and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template, implementing `IntoResponse`.
///
/// # Panics
///
/// Will panic if rendering the template fails (`unwrap()` on `html.render()`).
async fn down(mut cookie: CookieManager) -> impl IntoResponse {
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
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = PlayTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

/// Handler to move Rustbot **up** by one row in the grid.
///
/// Retrieves the current grid size and Rustbot coordinates from cookies,
/// decrements the `i` coordinate (row) by one, wrapping around to the maximum
/// if it reaches 0, updates the cookies, and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template, implementing `IntoResponse`.
///
/// # Panics
///
/// Will panic if rendering the template fails (`unwrap()` on `html.render()`).
async fn up(mut cookie: CookieManager) -> impl IntoResponse {
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
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = PlayTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

/// Handler to move Rustbot **right** by one row in the grid.
///
/// Retrieves the current grid size and Rustbot coordinates from cookies,
/// increments the `j` coordinate (column) by one, wrapping around to 0 if it
/// reaches the maximum, updates the cookies, and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template, implementing `IntoResponse`.
///
/// # Panics
///
/// Will panic if rendering the template fails (`unwrap()` on `html.render()`).
async fn right(mut cookie: CookieManager) -> impl IntoResponse {
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
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = PlayTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

/// Handler to move Rustbot **left** by one row in the grid.
///
/// Retrieves the current grid size and Rustbot coordinates from cookies,
/// decrements the `j` coordinate (row) by one, wrapping around to the maximum
/// if it reaches 0, updates the cookies, and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template, implementing `IntoResponse`.
///
/// # Panics
///
/// Will panic if rendering the template fails (`unwrap()` on `html.render()`).
async fn left(mut cookie: CookieManager) -> impl IntoResponse {
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
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = PlayTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

/// Handler to teleport Rustbot to specific coordinates `(i, j)` in the grid.
///
/// Retrieves the current grid size from cookies, sets Rustbot's coordinates
/// to the provided `i_teleport` and `j_teleport` values, updates the cookies,
/// and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
/// * `Path((i_teleport, j_teleport))` - The target coordinates provided in the URL path.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template, implementing `IntoResponse`.
///
/// # Panics
///
/// Will panic if rendering the template fails (`unwrap()` on `html.render()`).
async fn teleport(
    mut cookie: CookieManager,
    Path((i_teleport, j_teleport)): Path<(u32, u32)>,
) -> impl IntoResponse {
    // Initialize coordinates:
    let i_coord = i_teleport;
    let j_coord = j_teleport;

    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);

    // Add cookies
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = PlayTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

/// Handler to change the grid size.
///
/// Receives new grid dimensions from a submitted form (`MaxGridSizes`),
/// resets Rustbot's coordinates to `(0, 0)`, updates the cookies with
/// the new grid size, and renders the main HTML template.
///
/// # Arguments
///
/// * `cookie` - The `CookieManager` provided by Axum, used to read and update cookies.
/// * `Form(max_grid_sizes)` - The submitted form data containing the new grid dimensions.
///
/// # Returns
///
/// An `Html<String>` response containing the rendered template with Rustbot
/// reset at `(0, 0)` and the updated grid size.
///
/// # Panics
///
/// Will panic if rendering the template fails (`unwrap()` on `html.render()`).
async fn change_max(
    mut cookie: CookieManager,
    Form(max_grid_sizes): Form<MaxGridSizes>,
) -> impl IntoResponse {
    let grid_max_i = max_grid_sizes.change_max_i;
    let grid_max_j = max_grid_sizes.change_max_j;

    // Add cookies
    update_cookie(0, 0, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = PlayTemplate {
        rustbot_i: 0,
        rustbot_j: 0,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn code(mut cookie: CookieManager) -> impl IntoResponse {
    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);
    let (i_coord, j_coord) = get_rustbot_coordinates(&cookie);

    // Add cookies
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = CodeTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };
    Html(html.render().unwrap())
}

async fn user_code(
    mut cookie: CookieManager,
    Form(user_code): Form<UserCode>,
) -> impl IntoResponse {
    let user_code = user_code.user_code;

    // Retrieve cookies if already existing
    let (grid_max_i, grid_max_j) = get_grid_size(&cookie);
    let (mut i_coord, mut j_coord) = get_rustbot_coordinates(&cookie);

    for line in user_code.lines() {
        if line.contains("right") {
            if j_coord == grid_max_j - 1 {
                j_coord = 0;
            } else {
                j_coord += 1;
            }
        } else if line.contains("left") {
            if j_coord == 0 {
                j_coord = grid_max_j - 1;
            } else {
                j_coord -= 1;
            }
        } else if line.contains("down") {
            if i_coord == grid_max_i - 1 {
                i_coord = 0;
            } else {
                i_coord += 1;
            }
        } else if line.contains("up") {
            if i_coord == 0 {
                i_coord = grid_max_i - 1;
            } else {
                i_coord -= 1;
            }
        } else if line.contains("go to") {
            let re = Regex::new(r"go to \(([0-9]+) ?; ?([0-9]+)\)").unwrap();
            let matches = re.captures(line).unwrap();
            i_coord = matches[1].parse().unwrap();
            j_coord = matches[2].parse().unwrap();
        } else {
            panic!("Unknwon command: {line}");
        }
    }

    // Add cookies
    update_cookie(i_coord, j_coord, grid_max_i, grid_max_j, &mut cookie);

    // Create html response
    let html = CodeTemplate {
        rustbot_i: i_coord,
        rustbot_j: j_coord,
        grid_max_i,
        grid_max_j,
    };

    
    Html(html.render().unwrap())
}
