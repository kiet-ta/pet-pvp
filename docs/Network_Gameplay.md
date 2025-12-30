# Network Gameplay & Architecture

## 1. Client-Side Prediction (CSP)

### Context

In a networked game, waiting for the server to confirm movement results in input lag (equal to RTT). CSP allows the client to act immediately on input while waiting for server confirmation.

### Implementation

- **Input Buffer**: Client stores inputs (Move, Jump) in a `VecDeque<PlayerInput>` tagged with a `tick`.
- **Immediate Simulation**: Upon pressing a key, the client _immediately_ runs the movement logic (`shared_movement_logic`) on the local player entity.
- **Server Authority**: The server receives the input, processes it authoritatively, and sends back the true `position`, `velocity`, and the `last_processed_tick`.

## 2. Server Reconciliation

### Context

Since the client predicts the future, it may drift from the server (due to packet loss, nondeterminism, or cheating). Reconciliation corrects this drift.

### Implementation

When the Client receives a `ServerMessages::PlayerSync` for the local player:

1.  **Prune**: Remove inputs from the buffer that are older than `last_processed_tick` (Server has confirmed them).
2.  **Snap**: Reset the local player's `Transform` and `Velocity` to the Server's values.
3.  **Replay**: Re-run the movement logic for all remaining (pending) inputs in the buffer to get back to the "current" predicted time.

> **Note**: This ensures the client is always visually responsive but ultimately bound by the server's truth.

## 3. Gameplay Loop & State Management

### Problem

Previously, the game ran all systems (Input, Physics, Network) immediately upon launch. This wastes resources (physics running in Menu) and complicates flow control (e.g., handling Game Over).

### Solution: `AppState`

We utilize `bevy_state` to manage high-level game states.

```rust
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,       // Main Menu, UI only
    Lobby,      // Waiting for connection/players
    InGame,     // Active gameplay, Physics, Inputs active
    GameOver,   // Scoreboard, Reset options
}
```

### Transition Flow

1.  **Menu**: App starts here. User presses "Start".
2.  **Lobby**: Connect to Server. Wait for `PlayerConnected`.
3.  **InGame**: Transition when Game Start condition is met.
    - _Active Systems_: Physics, Player Input, Network Sync.
4.  **GameOver**: Transition on Win/Loss.
    - _Active Systems_: UI, Score display. Physics Paused.

### Architecture Rules

- **Physics**: Run ONLY in `InGame`.
- **Input**: `handle_input` runs ONLY in `InGame`.
- **Network**: Connection maintenance runs always, but Gameplay Sync runs ONLY in `InGame`.
