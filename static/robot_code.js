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
            const response = await fetch(
                "/user-code",
                {
                    method: "POST",
                    body: new URLSearchParams(formData),
                    headers: { "Content-Type": "application/x-www-form-urlencoded" }
                }
            )
            const body = await response.text();
            const re = /<table id="robot-grid">.+<\/table>/s;
            const grid = re.test(body);
            console.log(grid);
            document.getElementById("robot-grid").innerHTML = grid;
        } else {
            console.log(event);
            return;
        }
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