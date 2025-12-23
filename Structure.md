## ECS-driven Modular Architecture with some flavor of Layered + Feature-based design.

```
pet-pvp/ (Root)
├── Cargo.toml          # Workspace
├── crates/
│   ├── shared/         # "heart" of the game: Logic, Physics, Components
│   ├── server/         # Headless Server (Authority)
│   └── client/         # Game Client (Rendering, UI, Input)
├── assets/             # Sprites, Sounds
└── docs/
```

## Project Architecture Overview

This project is organized as a **multi-crate workspace** to clearly separate shared logic, server authority, and client experience.
The goal is to keep the game **deterministic**, **secure**, and **smooth** in a networked environment.

---

### `crates/shared/` — Source of Truth

This crate contains all **core game logic** that must run **identically on both the Client and the Server** to prevent desynchronization (state mismatch).

**Responsibilities:**

- Define shared data models
- Implement deterministic game systems
- Ensure consistent physics and rules across all environments

**Structure:**

- `components.rs`
  Shared ECS components such as `Position`, `Velocity`, `Health`.

- `systems/`
  Core gameplay logic including movement, collision handling, and damage calculation.

- `physics.rs`
  Configuration for `bevy_rapier2d` using a fixed timestep to ensure deterministic simulation.

- `protocol.rs`
  Definitions of network packets used for communication between Client and Server.

---

### `crates/server/` — The Authority

The server is the **authoritative source of truth** for the game state.
It validates player actions, manages connections, and prevents cheating.

**Key characteristics:**

- Runs in **headless mode**
- No rendering or window system
- Focused entirely on logic and validation

**Structure:**

- Headless setup
  Does not use `DefaultPlugins`.
  Only minimal plugins are enabled (e.g. `AssetPlugin`, `ScenePlugin`), with no window or rendering.

- `networking/`
  Handles socket connections and receives player input from clients.

- `world_authority.rs`
  Validates gameplay actions, such as:
  - Checking whether a shot is valid
  - Detecting illegal movement (e.g. speed hacking)

---

### `crates/client/` — The Experience

The client focuses on **visuals, input handling, and player experience**.
It reacts to server updates and presents a smooth, responsive game view.

**Structure:**

- `rendering/`
  Sprites, animations, and camera configuration.

- `ui/`
  HUD, menus, health bars, and other interface elements.

- `input.rs`
  Captures keyboard and mouse input and sends it to the server immediately
  (supports predictive input for rollback networking).

- `interpolation.rs`
  Smooths movement of other players using server state updates.

---

### Design Principles

- **Shared logic first**: Core rules live in `shared` to avoid duplication and desync.
- **Server authority**: The server validates all important actions.
- **Client-side smoothness**: Prediction and interpolation improve responsiveness without breaking consistency.

---

### Data Lifecycle

Input
--> MovementSystem
--> PhysicsSystem
--> SyncState
--> UI
