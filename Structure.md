### ECS-driven Modular Architecture with some flavor of Layered + Feature-based design.

```
src/
  main.rs

  core/             # foundation Layer
    mod.rs
    components.rs   # shared data (Position, Velocity, Health...)
    resources.rs    # game-wide states (Scoreboard, MatchState)
    state.rs        # menu, lobby, in-game

  gameplay/         # Logic Layer (The brain)
    mod.rs
    player.rs       # movement, input
    enemy.rs        # AI or mirror player for local PVP
    combat.rs       # attacks, damage systems
    physics.rs      # collisions, knockback

  networking/       # Networking Layer (The Nervous system)
    mod.rs
    sync.rs         # ECS replication state
    transport.rs    # WebSockets / QUIC etc.

  rendering/        # Presentation Layer (The eyes)
    mod.rs
    camera.rs
    animations.rs
    sprites.rs

  ui/              # User Interface Layer (The face)
    mod.rs
    hud.rs          # player health bars, timer
    menus.rs        # main menu, lobby

assets/
  sprites/
  sounds/
```

### Data Lifecycle

Input
--> MovementSystem
--> PhysicsSystem
--> SyncState
--> UI
