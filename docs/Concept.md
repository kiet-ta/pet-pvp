## Network

### Peer-to-peer Rollback (GGPO Style)

**Problem**: network allway has latency (ping). If I play a game as a action and I must waiting for server confirm that action, my feeling very bad. I will using _**Rollback Netcode(GGPO Style)**_

- Resolve By Rollback:
  1. Predict: when I click "Fire", client game will show screen Fire now (0 latency), dont need waiting Server. Yeh!!!!

  2. Check: After server send response. If Server confirm like "OK, Fire to enermy successfully", game will run smoothly. Ah ha !!!

  3. Rollback: If Server confirm "No, it your player was stund, cannot Fire", Client will rewind, set status "stund" and fast forward to the present. All of action just work in 1 frame so my eye hard to see that.

### Define Protocol

- Define Network Message Protocol between Client and Server

```Rust

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkMessage {
    // Client send action input to Server
    InputUpdate {
        frame: u32,
        action: crate::PlayerAction,
    },
    // Server send world state to Client
    WorldStateSync {
        frame: u32,
        player_positions: Vec<(u32, bevy::prelude::Vec2)>,
    },
}
```

### The Metaphor of Time Travel

- When using rollback netcode, think of your game as having the ability to "time travel." When a player makes an action, the game predicts the outcome and moves forward in time. If the server later indicates that the prediction was incorrect, the game "travels back in time" to the point of the action, adjusts the game state accordingly, and then fast-forwards back to the present moment. This metaphor helps conceptualize how rollback netcode manages latency and maintains a smooth gameplay experience.
