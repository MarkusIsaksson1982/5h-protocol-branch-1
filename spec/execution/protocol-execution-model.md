# Protocol Execution Model (PEM)

## Purpose
Defines how a 5H pipeline is executed step-by-step.

---

## Execution Loop

For each layer Hn:

1. Validate input against schema
2. Apply transformation
3. Validate output
4. Log result
5. Pass to next layer

---

## Canonical Interface

Each layer must implement:

```

execute(input_state) -> output_state
validate(input_state) -> bool

```

---

## Determinism Requirement

All layers must declare:

- deterministic: true | false

If false:
- Must include:
  - confidence score
  - alternative outputs (optional)

---

## Failure Handling

On failure:

- Retry (if soft)
- Rollback (if hard)
- Abort (if critical)
