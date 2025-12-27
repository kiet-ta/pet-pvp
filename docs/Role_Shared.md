# Refactoring: Shared Crate Architecture (Separation of Concerns)

This document explains the refactoring of the `shared` crate to resolve circular dependency and code duplication issues.

## The Problem

Previously, `lib.rs` and `protocol.rs` were intermingled:

- `lib.rs` contained Network Message definitions.
- `protocol.rs` tried to reference game logic.
- This led to scalability and maintenance issues (Spaghetti code).

## The New Structure

We apply the **"Contract & Machinery"** concept:

### 1. `protocol.rs` (The Contract)

This file acts as the "Common Language" between Client and Server. It is completely **pure**, meaning it does not contain game processing logic or Bevy Systems.

- **Content:**
  - Network Constants (`CHANNEL_ID`, `PROTOCOL_ID`).
  - Raw Data Structs (Data Transfer Objects - DTO) like `PlayerInput`.
  - Message Enums (`ClientMessages`, `ServerMessages`).
- **Characteristics:** Contains only Data definitions (Struct/Enum) and Serializable. Does not depend on `lib.rs`.

### 2. `lib.rs` (The Logic)

This file contains the Game Logic (Shared Logic) that both Client and Server need to run (e.g., physics, movement).

- **Content:**
  - ECS Components like `Player`, `Velocity`.
  - Resources managing logic state like `PlayerInputs` (Map entity -> input).
  - Systems like `player_movement_system`.
  - Bevy Plugin (`SharedPlugin`) to install these systems.
- **Characteristics:** Imports `protocol.rs` to understand input data structures, then applies logic to Entities.

## Dependency Graph

```mermaid
graph TD
    Client --> Shared
    Server --> Shared

    subgraph Shared Crate
        Lib[lib.rs (Logic)] --> Protocol[protocol.rs (Data Types)]
    end
```

This separation allows us to easily change the network protocol without breaking game logic, and vice versa.
