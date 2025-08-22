/**
 * @file robot_code.js
 * @brief Script for interacting with the robot grid in code mode.
 */

/**
 * @brief Handle click events (on grid or buttons).
 * 
 * @param {MouseEvent} event - The click event.
 */
async function onClick(event) {
    event.preventDefault();
    try {
        if (event.target.id == "button-form-code") {
            const form = document.querySelector("#user-code");
            const formData = new FormData(form);
            await fetch(
                "/user-code",
                {
                    method: "POST",
                    body: new URLSearchParams(formData),
                    headers: { "Content-Type": "application/x-www-form-urlencoded" }
                }
            );
        } else {
            console.log(event);
            return;
        }
        replaceGrid();
    } catch (e) {
        if (e instanceof TypeError && e.message == "NetworkError when attempting to fetch resource.") {
            console.log(e);
            alert("Could not reach server!");
        } else {
            console.log(e);
        }
    }
}

const button_form = document.getElementById("button-form-code");
button_form.addEventListener("click", onClick);

/**
 * @brief Reads cookies of document.
 */
function readCookies() {
    const cookies_array = document.cookie.split("; ");
    const x = cookies_array.find((row) => row.startsWith("i=")).split("=")[1];
    const y = cookies_array.find((row) => row.startsWith("j=")).split("=")[1];
    const max_x = cookies_array.find((row) => row.startsWith("max-i=")).split("=")[1];
    const max_y = cookies_array.find((row) => row.startsWith("max-j=")).split("=")[1];
    return [x, y, max_x, max_y];
}

/**
 * @brief Generate grid given robot's coordinate and size of grid.
 * 
 * @param {int} x_coord Robot's x coordinate.
 * @param {int} y_coord Robot's y coordinate.
 * @param {int} x_max Value max for x (so number of lines in the grid - 1).
 * @param {int} y_max Value max for y (so number of columns in the grid - 1).
 */
function generateGrid(x_coord, y_coord, max_x, max_y) {
    let grid = "";
    for (let x = 0; x < max_x; x++) {
        grid += "<tr>";
        for (let y = 0; y < max_y; y++) {
            if (x == x_coord && y == y_coord) {
                grid += `<td class="grid-cell" data-x='${x}' data-y='${y}'><img src='/static/robot.png' alt='Robot' class='image-responsive'></td>`;
            } else {
                grid += `<td class="grid-cell" data-x='${x}' data-y='${y}'></td>`;
            }
        }
        grid += "</tr>";
    }
    document.getElementById("robot-grid").innerHTML = grid;
}

/**
 * @brief Replace grid by newly generated one (with new robot's coordinates).
 */
function replaceGrid() {
    const [x_coord, y_coord, max_x, max_y] = readCookies();
    generateGrid(x_coord, y_coord, max_x, max_y);
}