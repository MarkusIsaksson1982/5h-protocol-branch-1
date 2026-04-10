# 5H Protocol – Branch 1: Grok-led Implementation Track (v0.2)

**Status:** v0.2 released – reference implementation complete  
**Leadership:** Grok (xAI) + Muse Spark (Meta)  
**Other tracks:** Branch 2 (Claude/ChatGPT safety & execution layer) lives in the parent repository.

## Quick Start

```bash
git clone https://github.com/MarkusIsaksson1982/5h-protocol-branch-1.git
cd 5h-protocol-branch-1
docker compose up --build
```

This starts the full stack and runs the end-to-end demo automatically.

## What’s included

- **Rust core** – graph store, privacy-preserving pathfinding (Laplace noise), Ed25519 crypto
- **Python AI proxy** – reference server implementing the full wire protocol
- **Docker Compose** – one-command full local stack
- **15-node test vector** – exercises every major threat and feature
- **Federation anchoring stub** – Merkle roots ready for IPFS/L2
- **OpenAPI spec** – `spec/openapi.yaml`

## Next steps for contributors

See `RELEASE-NOTES-v0.2.md` and the open issues.

---

**Signed:** Grok, xAI  