## The core idea (architecture-level, not buzzword-level)

In a system, you always have:

- data
- rules about that data
- multiple modules that want to touch both

SSOT says:

> One owner. Everyone else consumes.

That owner can be:

- a database
- a backend service
- a config file
- a state store
- a schema
- a contract

But never “whatever file I was in at the time”.

---

## Why SSOT exists (because humans copy-paste)

Without SSOT, you get:

- Frontend validating email one way
- Backend validating it another way
- Mobile app validating it a third way
- Bugs that only appear on Tuesdays

SSOT fixes:

- data inconsistency
- duplicated logic
- impossible debugging
- “works on my machine” architecture

Basically, it reduces chaos. Humans hate that, systems need it.

---

## Concrete examples (real-world, not fairy tales)

### 1. Database as SSOT

**Scenario**: User profile data

- Name, email, role stored in DB
- Backend reads/writes DB
- Frontend never “stores truth”, only displays it

DB = SSOT
Frontend cache ≠ truth

If the DB says the user is admin, congratulations, they are admin.

---

### 2. Backend API as SSOT

**Scenario**: Business rules

- Price calculation
- Discount rules
- Permissions

All logic lives in backend service.

Frontend:

- does not reimplement rules
- does not guess outcomes
- asks the backend

Backend API = SSOT
Frontend logic = UI sugar

---

### 3. State management in frontend

**Scenario**: React app

You pick:

- Redux store
- Zustand
- React Context

That store is the SSOT.

Bad architecture:

- component A keeps local state
- component B keeps its own version
- they disagree

Good architecture:

- one store
- components subscribe

State store = SSOT
Components = observers

---

### 4. Config & constants

**Scenario**: Environment values

- API base URL
- feature flags
- limits

Defined in:

- `.env`
- config service

Not:

- hardcoded in 12 files

Config = SSOT
Code = consumer

---

### 5. Schema / contract as SSOT

**Scenario**: Microservices or frontend-backend

- OpenAPI / Swagger
- GraphQL schema
- Protobuf

The schema defines:

- fields
- types
- constraints

Both sides follow it.

Schema = SSOT
Implementations = servants

---

## SSOT across layers (big picture)

```
Frontend
  ↓
Backend API  ←── Business Rules (SSOT)
  ↓
Database     ←── Data (SSOT)
```

Each layer has **its own SSOT**, not one global god-object.

That’s the part people mess up.

---

## Anti-patterns (how SSOT dies)

- “Let’s just duplicate it for performance”
- “Frontend can calculate it too”
- “We’ll keep a backup copy here”
- “It’s faster to hardcode”

All of these create:

- divergence
- sync bugs
- late-night debugging

---

## Rule of thumb (burn this into your brain)

If two places can **conflict**, you don’t have SSOT.
If you must update something in more than one place, you already lost.

---
