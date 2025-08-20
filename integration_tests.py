from bs4 import BeautifulSoup
import re
import requests
import sys
import subprocess
import time

local_address = "http://0.0.0.0:3000/"

def check_diff(received, expected):
    """
    Checks difference between two strings.
    First compares the lengths. If they are equal, compares character by
    character.

    Args:
        received (string): Received string.
        expected (string): Expected string.
    
    Returns:
        flag (boolean) : True if strings differ, False otherwise.
    """
    lr = len(received)
    le = len(expected)
    if lr != le:
        print(f"Lengths differ: {lr} (received) != {le} (expected)")
        return False
    flag = True
    for i in range(lr):
        ri = received[i]
        ei = expected[i]
        if ri != ei:
            print(f"Character {i}: {ri} (received) != {ei} (expected)")
            flag = False
    return flag

def find_robot_grid(r):
    """
    Find the table containing the robot using BeautifulSoup.
    Table should have the class 'robot-grid'.

    Args:
        r (request): request containing the table to analyze.

    Returns:
        table (list): table as a list of lists.
    """
    soup = BeautifulSoup(r.text, "html.parser")
    table_text = soup.find("table", id="robot-grid")
    table = []
    for line in table_text.find_all("tr"):
        l = []
        for col in line.find_all("td"):
            l.append(col)
        table.append(l)
    return table

def find_image_in_grid(table):
    """
    Find the robot in the grid parsed using find_robot_grid().

    Args:
        table (list): see result of find_robot_grid().

    Returns:
        x (int): robot's x coordinate in the table.
        y (int): robot's y coordinate in the table.
    """
    found = False
    for l, line in enumerate(table):
        for c, col in enumerate(line):
            if len(col.find_all("img")) == 1:
                if not(found):
                    found = True
                    x = l
                    y = c
                else:
                    raise Exception(f"Image found twice! ({x}, {y}) and now ({l}, {c})")
    return x, y

def final_position(moves, x_max = 4, y_max = 4):
    """
    Given a list of the moves, this function computes the final coordinates.
    Important note: the list always needs to start with a 'reset' to ensure 
    replicable tests!

    Args:
        moves (list): list of moves done by the robot. Always start with
            'reset', and can contain the following elements in any order
            (and repeated as many times as you want):
            - 'reset'
            - 'up'
            - 'down'
            - 'left'
            - 'right'
            - 'coords/x/y' with x and y as integers
        x_max (int): Value max for x (so number of lines in the grid - 1).
        y_max (int): Value max for y (so number of columns in the grid - 1).
    
    Returns:
        x (int): robot's final x coordinate in the table.
        y (int): robot's final y coordinate in the table.
    """
    if moves[0] != 'reset':
        raise ValueError(f"moves should always start with 'reset' and not '{moves[0]}'")
    
    x = 0
    y = 0
    for m in moves:
        if m == "reset":
            x = 0
            y = 0
        elif m == "up":
            x -= 1
            if x == -1:
                x = x_max
        elif m == "down":
            x += 1
            if x == x_max + 1:
                x = 0
        elif m == "left":
            y -= 1
            if y == -1:
                y = y_max
        elif m == "right":
            y += 1
            if y == y_max + 1:
                y = 0
        elif re.search("coords/[0-9]+/[0-9]+", m):
            l = re.split("coords/([0-9]+)/([0-9]+)", m)
            x = int(l[1])
            y = int(l[2])
        else:
            raise ValueError(f"Unknown move: {m}")
    return x, y

def test_moves(moves):
    """
    Test a sequence of moves by sending the matching requests to the
    server, finind the robot in the final response, and comparing it to the
    theorical answer.

    Args:
        moves (list): list of moves done by the robot. Always start with
            'reset', and can contain the following elements in any order
            (and repeated as many times as you want):
            - 'reset'
            - 'up'
            - 'down'
            - 'left'
            - 'right'
            - 'coords/x/y' with x and y as integers

    Returns:
        (boolean): True if last position matches the expected one.
    """
    expected_x, expected_y = final_position(moves)
    session = requests.Session()
    for m in moves:
        if re.search("coords", m):
            r = session.get(local_address + m)
        else:
            r = session.post(local_address + m)
    final_x, final_y = find_image_in_grid(find_robot_grid(r))
    if (final_x != expected_x) or (final_y != expected_y):
        print("Final position doesn't match expected position!")
        print(f"moves: {moves}")
        print(f"expected x: {expected_x} | final x: {final_x}")
        print(f"expected y: {expected_y} | final y: {final_y}")
        return False
    return True
    

def main():
    """
    Integration tests.
    With previously opened server:
    - TEST 1: ask for CSS template, PNG robot image, JS script and check if
              matches the saved ones.
    - TEST 2: ask for random requests.
    - TEST 3: tries many moves sequences.

    Returns:
        (boolean): True if all tests passed, otherwise False.
    """
    print("------------------------------------------------------------------")
    print("TEST 1 : templates")
    print("------------------")
    # CSS template
    flag11 = True
    r_css = requests.get(local_address + 'static/template.css')

    with open("static/template.css", "r") as f:
        expected_css = f.read()
    
    if r_css.status_code != requests.codes.ok:
        print(f"Bad status code: {r_css.status_code}")
        flag11 = False

    if not check_diff(r_css.text, expected_css):
        print("Problem with CSS file")
        flag11 = False

    ct = r_css.headers["Content-Type"]
    if ct != "text/css":
        print(f"Wrong Content-Type: {ct} (received) != text/css (expected)")
        flag11 = False

    cl = int(r_css.headers["Content-Length"])
    lecss = len(expected_css)
    if cl != lecss:
        print(f"Wrong Content-Length: {cl} (received) != {lecss} (expected)")
        flag11 = False

    if flag11:
        print("> CSS template OK!")
    
    # Robot PNG image
    flag12 = True
    r_png = requests.get(local_address + "static/robot.png")

    with open("static/robot.png", "rb") as f:
        expected_robot_png = f.read()

    if r_png.status_code != requests.codes.ok:
        print(f"Bad status code: {r_png.status_code}")
        flag12 = False
    
    if not check_diff(r_png.content, expected_robot_png):
        print("Problem with robot PNG file")
        flag12 = False

    ct = r_png.headers["Content-Type"]
    if ct != "image/png":
        print(f"Wrong Content-Type: {ct} (received) != image/png (expected)")
        flag12 = False

    cl = int(r_png.headers["Content-Length"])
    lecss = len(expected_robot_png)
    if cl != lecss:
        print(f"Wrong Content-Length: {cl} (received) != {lecss} (expected)")
        flag12 = False

    if flag12:
        print("> Robot PNG image OK!")

    # JS script
    flag13 = True
    r_js = requests.get(local_address + 'static/robot.js')

    with open("static/robot.js", "rb") as f:
        expected_js = f.read()

    if r_js.status_code != requests.codes.ok:
        print(f"Bad status code: {r_js.status_code}")
        flag13 = False
    
    if not check_diff(r_js.content, expected_js):
        print("Problem with JavaScript file")
        flag13 = False

    ct = r_js.headers["Content-Type"]
    if ct != "text/javascript":
        print(f"Wrong Content-Type: {ct} (received) != text/javascript (expected)")
        flag13 = False

    cl = int(r_js.headers["Content-Length"])
    lecss = len(expected_js)
    if cl != lecss:
        print(f"Wrong Content-Length: {cl} (received) != {lecss} (expected)")
        flag13 = False

    if flag13:
        print("> JS script OK!")

    flag1 = flag11 and flag12 and flag13
    if flag1:
        print("OK!")
    print("------------------------------------------------------------------")

    print("------------------------------------------------------------------")
    print("TEST 2 : Random requests")
    print("------------------------")
    flag2 = True
    r_rand = requests.get(local_address + '.well-known/appspecific/com.chrome.devtools.json')
    
    if r_rand.status_code != requests.codes.not_found:
        flag2 = False

    if flag2:
        print("OK!")
    
    # TODO: add unknown requests
    print("------------------------------------------------------------------")

    print("------------------------------------------------------------------")
    print("TEST 3 : moving!")
    print("----------------")

    moves_to_test = [
        ["reset", "up", "up", "up"], # Move up 3 times
        ["reset", "right", "right", "right"], # Move right 3 times
        ["reset", "left", "left"], # Move left 2 times
        ["reset", "down"], # Move down once
        ["reset", "up"], # Move up once
        ["reset", "up", "up", "up", "up", "up", "up"], # Test top boundary wrap-around
        ["reset", "right", "right", "right", "right", "right", "right"], # Test right boundary wrap-around
        ["reset", "down", "down", "down", "down", "down", "down"], # Test bottom boundary wrap-around
        ["reset", "left", "left", "left", "left", "left", "left"], # Test left boundary wrap-around
        ["reset", "up", "right", "down", "left", "up", "up", "right", "down", "down", "left", "left"], # Long random path
        ["reset", "up", "right", "down", "left"], # Full loop (back to origin)
        ["reset", "up", "right", "up", "right", "down", "left", "down", "left"], # Zigzag pattern
        ["reset", "right", "right", "left", "left"], # Horizontal back and forth
        ["reset", "up", "up", "down", "down"], # Vertical back and forth
        ["reset", "up", "right", "reset", "down", "left"], # Multiple resets (should always return to (0,0))
        ["reset", "up", "up", "right", "right", "down", "down", "left", "left", "up", "up", "right", "down", "left"], # Random long path with multiple wrap-arounds
        ["reset", "up", "down", "left", "right", "up", "up", "right", "down", "left"], # Random moves with redundant steps (tests stability)
        ["reset", "up", "down", "right", "left", "down", "up", "left", "right"], # Stay in place (all moves cancel each other)
        ["reset", "up", "right", "down", "left", "up", "right", "down", "left"], # Diagonal loop (simulate moving around corners)
        ["reset", "up", "up", "up", "up", "up", "right", "right", "right", "right", "right",
        "down", "down", "down", "down", "down", "left", "left", "left", "left", "left",
        "up", "up", "up", "up", "up", "right", "right", "right", "right", "right",
        "down", "down", "down", "down", "down", "left", "left", "left", "left", "left"], # Edge surfing (go around the grid in a square pattern twice)
        ["reset", "coords/2/3"],  # Simple click move
        ["reset", "coords/2/3", "up", "left"],  # Click then move
        ["reset", "up", "right", "coords/0/0"],  # Move then click back to origin
        ["reset", "coords/1/1", "coords/1/1"],  # Click same spot twice (should stay)
        ["reset", "coords/3/3", "coords/0/0", "coords/2/2"],  # Multiple clicks
        ["reset", "coords/4/4", "left", "left", "coords/3/1"],  # Click then move then click
        ["reset", "coords/2/2", "down", "right", "coords/1/1", "left", "coords/0/0"],  # Mixed path
        ["reset", "coords/0/1", "coords/1/1", "coords/1/0", "coords/0/0"],  # Clockwise around center
        ["reset", "coords/4/4", "coords/4/0", "coords/0/0"],  # Wrap-around via clicks
        ["reset", "coords/2/2", "reset", "coords/3/3"],  # Reset between clicks
        ["reset", "coords/0/0", "coords/0/0", "coords/0/0", "right", "right",
         "right", "right", "right", "coords/5/0", "down", "down", "down",
         "coords/5/3", "coords/5/3", "left", "left", "up", "reset",
         "coords/2/2", "right", "coords/3/2", "down", "coords/3/3", "reset",
         "coords/1/1", "right", "right", "down", "down", "left", "up",
         "coords/2/2", "coords/2/2", "coords/2/2", "reset", "right", "right",
         "right", "right", "down", "down", "coords/4/2", "up", "left", "left",
         "coords/2/1",] # ALL
    ]

    flag3 = True
    for moves in moves_to_test:
        if not test_moves(moves):
            flag3 = False
    
    if flag3:
        print("OK!")
    print("------------------------------------------------------------------")

    print("------------------------------------------------------------------")
    print("TEST 4 : change grid size")
    print("-------------------------")
    flag4 = True
    # Normal change
    new_max_i = 3
    new_max_j = 8
    r_change_grid = requests.post(
        local_address + 'change-max',
        data={
            "change_max_i": new_max_i,
            "change_max_j": new_max_j
        }
    )
    
    if r_change_grid.status_code != requests.codes.ok:
        print(f"Bad status code: {r_change_grid.status_code}")
        flag4 = False

    new_grid = find_robot_grid(r_change_grid)
    if len(new_grid) != new_max_i:
        print(f"Wrong number of lines!\nExpected: {new_max_i} | Received: {len(new_grid)}")
        flag4 = False
    if len(new_grid[0]) != new_max_j:
        print(f"Wrong number of columns!\nExpected: {new_max_j} | Received: {len(new_grid[0])}")
        flag4 = False
    
    if flag4:
        print("> Change size for inbounds values OK!")
    
    # Check if big values changed back to 20
    new_max_i = 99
    new_max_j = 5
    session = requests.Session()
    r = session.post(
        local_address + 'change-max',
        data={
            "change_max_i": new_max_i,
            "change_max_j": new_max_j
        }
    )
    r = session.get(local_address + 'right')
    new_grid = find_robot_grid(r)
    if len(new_grid) != 20:
        print(f"Max_i (> 20) should be forced back to 20 and not {len(new_grid)}!")
        flag4 = False
    
    if flag4:
        print("> Values > 20 set back to 20 OK!")
    
    # Check string instead of 
    new_max_i = 'Misty'
    new_max_j = 5
    session = requests.Session()
    r = session.post(
        local_address + 'change-max',
        data={
            "change_max_i": new_max_i,
            "change_max_j": new_max_j
        }
    )
    r = session.get(local_address + 'right')
    new_grid = find_robot_grid(r)
    if len(new_grid) != 5:
        print(f"Max_i (not a number) should be forced back to 5 and not {len(new_grid)}!")
        flag4 = False
    
    if flag4:
        print("> Absurd values set back to 5 OK!")
    

    if flag4:
        print("OK!")
    print("------------------------------------------------------------------")


    return flag1 and flag2 and flag3 and flag4

if __name__ == '__main__':
    # open server
    with open("server.log", "w") as server_log_file:
        server = subprocess.Popen(
            ["cargo", "run"],
            stdout=server_log_file, # Capture stdout
            stderr=server_log_file  # Capture stderr
        )
        print("--- SERVER OPENED! ---")

    # wait for server to start
    time.sleep(3)

    try:
        # launch tests
        flag = main()
    except:
        print("\n------------------------------------------------------------------")
        print("EXCEPTION RAISED!\nServer logs:")
        print("-------------------------------")
        with open("server.log", "r") as server_log_file:
            print(server_log_file.read())
        print("------------------------------------------------------------------")
        print("\n")
        raise
    finally:
        # terminates server
        server.terminate()
        server.wait()
        print("--- SERVER CLOSED! ---")

    # return
    if flag:
        print("All tests passed!")
        sys.exit(0)
    else:
        sys.exit(1)