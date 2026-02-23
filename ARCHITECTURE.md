# Architecture Guardrails

These rules are enforced to keep the codebase scalable and maintainable.

## 1. Domain Never Depends on Anything
- The domain crate must not depend on any other crate except contracts and shared types.
- Enforced by CI: `cargo check -p domain --no-default-features`

## 2. Contracts Contain Only Traits
- No structs, no impls, no logic.
- Only traits, DTOs, and error enums.

## 3. Infrastructure Is Only Used via Contracts
- Never import infrastructure types directly outside the app crate.
- Always depend on contracts traits.

## 4. Only App Constructs Concrete Types
- Only the app crate is allowed to call constructors (e.g., `SeededRng::new()`).
- All other crates receive dependencies via traits/interfaces.

## 5. Plugins Attach Only at Application Layer
- Plugins must implement `application::Plugin` and register via the application crate.

## 6. No Cyclic Dependencies
- Engine and infrastructure must never depend on each other directly.
- Use contracts for all cross-layer communication.

---
**Violations will be caught in code review and CI.**