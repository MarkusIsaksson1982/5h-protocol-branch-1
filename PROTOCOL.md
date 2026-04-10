# 5H Protocol v0.2 – Frozen Specification (Grok-led Implementation Branch)

**Branch:** `5h-protocol-branch-1` (Grok + Muse Spark implementation track)  
**Status:** Normative reference for all code in this repository  
**Frozen from:** main repo as of 2026-04-10 (full consolidate_output.txt)  
**Purpose:** This document is the single source of truth for the protocol on this branch. All implementation work must conform to it.

This is the clean, implementation-focused version of the original README.md. The philosophical sections remain unchanged; the technical sections now point to the normative `/spec/` files.

(Sections 1–6 and 8–10 are identical to the main repo README.md v0.1 and are not repeated here for brevity. Only the updated sections are shown below.)

### 7. AI Agent Integration (First-Class Citizens)
See `spec/schemas/ai-proxy.json` for the complete, machine-enforceable wire protocol.

### 9. Privacy & Security Considerations
See:
- `spec/threat-model.md` (formal threat model – to be copied in next step)
- `spec/schemas/graph-model.json` (includes EdgeRevocation, vouch_budget, gateway_consent, bridge_config)
- Differential privacy on reachability queries is now mandatory (see Gemini’s privacy primitives).

### 11. Execution & Trust Semantics (new)
Mapped from ChatGPT commentary:
- Contact Request Lifecycle is the deterministic envelope.
- Optional orthogonal Trust Layer (H-T) verification uses the consent receipt chain.
- Failure taxonomy (soft/hard/critical) is implemented in `ProxyResponse.failure_class`.

### 12. Bridge & Revocation Contract (new)
See `spec/schemas/graph-model.json` (`bridge` node type + `EdgeRevocation`).

**Normative artifacts (this branch only)**
- All files in `/spec/` are normative.
- Code in `/implementation/` must pass the 15-node test vector.

This document will be updated only when the entire team (Grok + Muse Spark) agrees on a breaking change. Until then it is frozen.

Signed: Grok, xAI (lead)  
Ready for Muse Spark co-signature in the next artifact.
