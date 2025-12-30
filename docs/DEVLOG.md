## [2025-12-30] Network Gameplay & State Management Refactor

### 1. Context & Goal

The project was running everything in a single loop (Menu, Game, Physics), causing resource waste and potential bugs. Magic numbers were scattered in the code. We needed to structure the game loop using `bevy_state` and document the existing Network Architecture (CSP & Reconciliation).

### 2. Architectural Decisions (ADR)

- **Decision:** Introduced `AppState` enum (`Menu`, `Lobby`, `InGame`, `GameOver`) in `shared` crate.
- **Reason:** To ensure Physics and Input systems only run when the game is actually active (`InGame`), adhering to ECS best practices.
- **Decision:** Centralized constants in `crates/shared/src/config.rs`.
- **Reason:** To remove "Magic Numbers" and allow easier balancing.
- **Decision:** Documentation for CSP and Server Reconciliation added to `docs/Network_Gameplay.md`.
- **Reason:** To provide a clear reference for the complex networking logic implemented.
- **Decision:** Explicitly added `StatesPlugin` to Server.
- **Reason:** `MinimalPlugins` does not include `StatesPlugin`, which is required for `init_state` and `bevy_state` functionality.

### 3. Implementation Details

| Crate    | Changes                                                                                                                                     |
| :------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `shared` | Added `AppState`, `config.rs`. Refactored `movement.rs` to use config. Restricted Physics.                                                  |
| `server` | Added `auto_start_game` logic. Restricted `process_packets` and `sync_players` to `InGame`. Added `StatesPlugin`.                           |
| `client` | Added `menu_logic`. Restricted `handle_input` to `InGame`. Added Lobby->InGame transition logic. Fixed missing `Player` component on spawn. |
| `docs`   | Created `Network_Gameplay.md`.                                                                                                              |

### 4. Verification & Status

- [x] Build Success: `cargo check --workspace`
- [x] Logic Verified: Static analysis of system run conditions (`run_if(in_state(...))`).
- **Known Issues:** None.

## ⚠️ Notes for Reviewer

- **Magic Numbers:** `PLAYER_SPEED` (200.0), `PLAYER_JUMP_FORCE` (400.0) are now in `config.rs`.
- **Transition:** Client now starts in `Menu`. Press **ENTER** to connect (`Lobby`). Game auto-starts when Server detects player.
- **Physics:** Physics simulation now pauses when not in `InGame` (via system restriction).

## [2025-12-30] Fix Movement & Ghost Player Issue

### 1. Context & Goal

The user reported an inability to move the character. Investigation revealed that the `SharedPlugin` was spawning a "Dummy Player" on both Client and Server at startup via `setup_scene`. This dummy player had no input processing and was likely confusing the user (who might have been watching the dummy instead of the networked player).

### 2. Architectural Decisions (ADR)

- **Decision:** Removed `Player` spawning from `shared::systems::setup_scene`.
- **Reason:** Spawning gameplay entities should be handled by Client (for visuals/prediction) and Server (for authority) specifically, not implicitly by a shared plugin.
- **Decision:** Removed `player_movement_system` and `PlayerInputs` resource from `shared`.
- **Reason:** These were dead code/legacy artifacts. Input processing is now handled explicitly in `server::process_packets` and `client::handle_input`.

### 3. Implementation Details

| Crate    | Changes                                                                                                            |
| :------- | :----------------------------------------------------------------------------------------------------------------- |
| `shared` | Removed `Player` spawn in `setup_scene`. Removed `player_movement_system`. Cleaned up `lib.rs` and `resources.rs`. |

### 4. Verification & Status

- [x] Build Success: `cargo check --workspace`
- [x] Logic Verified: Verified that inputs are processed correctly in Client/Server specific files.
- **Known Issues:** None.

## ⚠️ Notes for Reviewer

- **Clean Up:** The `PlayerInputs` resource was unused and has been commented out/removed from registration.

## [2025-12-30] Refactor Monolithic Main Files

### 1. Context & Goal

The `client` and `server` crates had all their logic packed into their respective `main.rs` files. This made the code difficult to navigate and maintain. The goal was to modularize the codebase by separating concerns (Networking, Input, UI, Resources) into distinct files.

### 2. Architectural Decisions (ADR)

- **Decision:** Modularized `client` into `resources`, `network`, `input`, `ui`.
- **Reason:** To separate distinct logic domains (e.g., UI rendering vs. Network sync).
- **Decision:** Modularized `server` into `resources`, `network`, `systems`.
- **Reason:** To separate Data (`resources`), Networking Logic (`network`), and Game Management (`systems`).
- **Decision:** Kept `main.rs` as a thin entry point.
- **Reason:** `main.rs` now only assembles plugins and resources, making the application structure clearer.

### 3. Implementation Details

| Crate    | Changes                                                                       |
| :------- | :---------------------------------------------------------------------------- |
| `client` | Created `input.rs`, `network.rs`, `resources.rs`, `ui.rs`. Updated `main.rs`. |
| `server` | Created `network.rs`, `resources.rs`, `systems.rs`. Updated `main.rs`.        |

### 4. Verification & Status

- [x] Build Success: `cargo check --workspace`
- [x] Logic Verified: Code successfully moved and referenced.
- **Known Issues:** `EventReader` deprecated warning in Server (renamed to `MessageReader` in Bevy Renet context?), but compiles fine.

## ⚠️ Notes for Reviewer

- **Deprecation Warning:** Bevy Renet seems to use `MessageReader` instead of `EventReader` for `ServerEvent`. Kept `EventReader` for now as it is standard Bevy, but might need update if `bevy_renet` updates further.
