Bevy is **Data-Oriented**

## What is Data-Oriented Programming?

That is a Programming Paradigm that focuses on the way data is organized and accessed in memory, rather than the traditional Object-Oriented approach that centers around objects and their behaviors. In Data-Oriented Programming, the emphasis is on optimizing data layout and access patterns to improve performance, especially in scenarios involving large datasets or real-time applications like games.

Think of the CPU not as a "brain," but as a super-fast factory worker. This worker hates moving around; they want materials to come to them on a conveyor belt, one after another.

**In DOD (used by Bevy/ECS), we split objects apart.**

**The Setup**: We store ingredients by type on long shelves (Arrays).

**Shelf 1 (Positions)**: [Pos A, Pos B, Pos C, Pos D...]

**Shelf 2 (Health)**: [Health A, Health B, Health C...]

**The Task**: "Update positions."

**The Process**: The CPU grabs the entire "Position Shelf" and sprints down the line: Update, Update, Update, Update.

**The Result**: **Cache Locality**. The CPU predicts the next data is right next to the current one, so it runs at maximum speed.

## Why do Bevy and Rust choose DOD?

- **Altar speed**: Modern CPUs hate jumping around in memory. They like to process sequential arrays. DOD helps games run 10-100 times faster than OOP in situations with a lot of units.

- **Parallelism**:
  **OOP**: It is very difficult to run in parallel because Objects are tangled, calling each other (Coupling), and prone to data conflicts (Race Condition).
  **DOD**: Because the data is separate (Position separate, Health separate), Bevy can automatically divide the work: Core 1 processes Position, Core 2 processes Health at the same time without fear of collision.

**Question**:

- The question is, if the player fields are all cut separately and saved in a separate ram area in list form to run linearly, will there be any difference if a piece of data is deleted?
  - Answer: 2 techniques are used to handle deletions:
    1. Archetype Swapping: When an entity is deleted, its data is swapped with the last entity in the array, and then the last entity is removed. This keeps the array compact and avoids gaps.
    2. Swap Removal: This technique involves swapping the entity to be deleted with the last entity in the array and then removing the last entity. This way, the array remains contiguous, and there are no gaps left by deletions.

## Example

## Archetypes - Groups of Components that define Entity types.

### Archetype A (Player)

has `{Position, Velocity, Player, Health}`

### Archetype B (Monster)

has `{Position, Velocity, AI}`

---

## In RAM, Bevy will create a table for each Archetype:

### table A (for Player)

| Row Index | Entity ID | Position (Vec3) | Velocity (Vec3) | Health (f32) |
| :-------: | :-------: | :-------------: | :-------------: | :----------: |
|     0     |  ID: 101  |     (0,0,0)     |     (1,0,0)     |     100      |
|     1     |  ID: 205  |    (10,5,0)     |     (0,2,0)     |      80      |
|     2     |  ID: 300  |       ...       |       ...       |     ...      |

---

### table B (for Monster)

| Row Index | Entity ID | Position (Vec3) | Velocity (Vec3) |  AI   |
| :-------: | :-------: | :-------------: | :-------------: | :---: |
|     0     |  ID: 50   |     (5,5,0)     |     (0,0,0)     | Aggro |

---

## Metadata Map

How does Bevy know where Entity ID 205 is located to get the data? Bevy keeps a secret "ledger" (Hash Map or Sparse Set) to map:

**Entity ID 205 -> { Archetype: A, Row: 1 }**

When you call query.get(entity_id), Bevy looks up this ledger, jumps to Table A, row number 1 to get the data. This process is extremely fast (O(1)).

## Swap Removal - resolve deletion gaps

- Suppose Table A has 3 people: [Player 1, Player 2, Player 3]. You want to delete Player 2 (in the middle, index 1).

- If deleting normally (shifting the array - Shift): [Player 1, (empty), Player 3] -> Move Player 3 back -> [Player 1, Player 3]. => Bad: Because you have to copy/shift memory, very slow if the array is long.

- How Bevy does it (Swap Removal):
  - Get the Last element (Player 3) and put it in the place of the deleted one (Player 2's position).
  - Update the "Address Book": Now Entity ID Player 3 is in Row 1.
  - Cut the tail of the array by 1 unit.

- Result:
  - The array is still dense, no holes.
  - The order is reversed (Player 3 jumps to the front), but ECS doesn't care about the order, it only cares about the data being complete and correct. --> **THAT IS OVERIDE**

Deletion speed is O(1) - Super fast even for arrays with 1 million elements.
