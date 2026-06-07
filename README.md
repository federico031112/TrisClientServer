# Tris (Tic-Tac-Toe) - Rust Client-Server Game

A simple multi-threaded client-server command-line application that allows two different clients to play Tris (Tic-Tac-Toe) over a network.

## Project Structure

The project is divided into two main components:
* **Server:** Manages connections, game state, and coordinates moves between players.
* **Client:** Connects to the server, displays the board, handles player input, and sends moves.

---

## Game Flow & Protocol

1. **Initialization:** The server must be started first. Once running, it listens for incoming connections.
2. **Connection:** Two separate clients connect to the server. If a connection fails, the client returns an error.
3. **Player Assignment:** Upon successful connection, the server assigns a unique ID (`1` or `2`) to each client to determine Player 1 and Player 2.
4. **The Match:**
    * **Player 1** starts the game, prints the board, makes the first move, and sends the coordinates (`row column`) to the server.
    * After every move, the client verifies the state of the board locally to check for a win.
    * The client then waits for the server to send Player 2's move, after which it checks if it has lost.
    * **Player 2** follows the same logic but starts the game by waiting for Player 1's first move.
5. **Termination:** The game loop runs continuously until a player wins, loses, or disconnects.

---

## Server Architecture & Threading

The server handles concurrency by spawning dedicated threads:
* For each accepted client connection, the server spawns a new thread via `thread::spawn`.
* Shared state between threads (such as the game board and player readiness) is synchronized using thread-safe smart pointers and primitives (`Arc<Mutex<Shared>>`).
* Depending on the client's assigned ID, the thread logic dynamically switches to listen for the correct turn sequence.

---

## Project Status

> ⚠️ **Note:** This is my first project exploring client-server synchronization and multithreading in Rust. While fully functional in local environments, the synchronization relies on a simple loop/sleep polling mechanism and is open to future refactoring (e.g., using channels or condition variables).