### GGPO Style

**Problem**: network allway has latency (ping). If I play a game as a action and I must waiting for server confirm that action, my feeling very bad. I will using _**Rollback Netcode(GGPO Style)**_

- Resolve By Rollback:
  1. Predict: when I click "Fire", client game will show screen Fire now (0 latency), dont need waiting Server. Yeh!!!!

  2. Check: After server send response. If Server confirm like "OK, Fire to enermy successfully", game will run smoothly. Ah ha !!!

  3. Rollback: If Server confirm "No, it your player was stund, cannot Fire", Client will rewind, set status "stund" and fast forward to the present. All of action just work in 1 frame so my eye hard to see that.

### Change Folder to Plugins
