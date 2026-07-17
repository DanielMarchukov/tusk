# Tusk — ITCH 5.0 Order Book Engine

## Rust Coding Standards

### Error Handling
- **Never use `.unwrap()` or `.expect()` in library code.** Use `?` with proper error types.
- `.unwrap()` is acceptable only in tests and benchmarks.
- Use `thiserror` for library error types. Define specific error variants, not catch-all strings.
- Use `anyhow` only in `main.rs` / CLI binary code, never in the library.
- Return `Result<T, E>` from any function that can fail. Do not panic.

```rust
// Bad
let value = map.get(&key).unwrap();

// Good
let value = map.get(&key).ok_or(Error::OrderNotFound(key))?;
```

### No Unnecessary Cloning
- Never `.clone()` to satisfy the borrow checker without understanding why.
- If you need to clone, add a comment explaining why the borrow doesn't work.
- Prefer borrowing (`&T`, `&str`) over owning (`T`, `String`) in function signatures.
- Use `Cow<'a, str>` when a function sometimes borrows, sometimes owns.

### Clippy — Pedantic
- All code must pass `clippy::pedantic` and `clippy::nursery`.
- Use `#[allow(clippy::specific_lint)]` with a comment when a pedantic lint is a false positive.
- Never blanket-allow clippy lints at the crate level.

```rust
// In lib.rs
#![warn(clippy::pedantic, clippy::nursery)]
```

### Unsafe Code
- Every `unsafe` block must have a `// SAFETY:` comment explaining why it is sound.
- Unsafe must be confined to the smallest possible scope.
- Document the invariants that the caller must uphold.
- Prefer safe abstractions — only use unsafe when there is a measurable performance benefit backed by benchmarks.

```rust
// SAFETY: `data` is guaranteed to be at least 8 bytes by the message
// length check in `dispatch()`. The bytes are valid for a u64 read
// as ITCH uses fixed-width fields at known offsets.
let order_id = unsafe { data.as_ptr().add(11).cast::<u64>().read_unaligned() };
```

### Type System
- Use newtypes for domain concepts: `Price(u32)`, `OrderId(u64)`, `Size(u32)`.
- Use enums over booleans: `Side::Bid` / `Side::Ask` not `is_buy: bool`.
- Use `#[must_use]` on functions that return values the caller should not ignore.
- Use `#[non_exhaustive]` on public enums that may gain variants.

### Memory and Performance
- No heap allocation in hot paths unless benchmarks prove it doesn't matter.
- Prefer stack-allocated fixed-size arrays over `Vec` when the size is known.
- Use `#[repr(C)]` when memory layout matters (parsing, cache alignment).
- Use `#[repr(align(64))]` for cache-line-aligned structs where appropriate.
- Prefer `#[inline]` only on small functions called across module boundaries in hot paths. Do not blanket-inline everything.
- Use `MaybeUninit` over zero-initialization when the value will be immediately overwritten and the savings are measurable.

### Testing
- Unit tests in the same file as the code (`#[cfg(test)] mod tests`).
- Integration tests in `tests/` for end-to-end replay scenarios.
- Property-based tests with `proptest` for parser correctness (fuzz-like coverage).
- Every public function has at least one test.
- Test both happy path and error cases.
- Benchmarks in `benches/` using criterion.rs. Each optimization must have before/after numbers.

### Documentation
- All public types and functions have `///` doc comments.
- Doc comments describe **what** and **why**, not **how** (the code shows how).
- Include `# Examples` in doc comments for non-obvious APIs.
- No doc comments on private internals unless the logic is genuinely tricky.

### Naming
- Follow Rust API guidelines: https://rust-lang.github.io/api-guidelines/
- Types: `PascalCase`. Functions: `snake_case`. Constants: `SCREAMING_SNAKE_CASE`.
- Avoid abbreviations except well-known ones (`id`, `msg`, `buf`, `len`).
- Iterator methods: `.iter()`, `.into_iter()`, `.iter_mut()` conventions.
- Conversion methods: `as_*` (cheap, borrowed), `to_*` (expensive), `into_*` (consuming).

### Dependencies
- Minimize dependencies. Each dep is a supply chain risk and compile time cost.
- Justify non-obvious dependencies in commit messages.
- Pin major versions in `Cargo.toml`.
- No `features = ["full"]` — enable only what you need.

### Git Discipline
- Prefix commit messages with the Taskwarrior task ID: `[54] Add NonCrossTrade struct`.
- Branch per task: `<id>/<short-slug>` (e.g. `54/non-cross-trade-struct`).
- Each optimization pass is a separate commit with benchmark numbers in the commit message.
- Commit messages explain **why**, not **what** (the diff shows what).
- Keep commits atomic — one logical change per commit.

### CI Workflows
- Always verify the default branch name with `git branch --show-current` before writing CI configs. Never assume `main` or `master`.
- Generated Rust files must end with a trailing newline.
- Bench/test stubs must pass `clippy::pedantic` — avoid triggering const fn or dead code lints in scaffolding.

---

## Architecture

- `src/parser/` — ITCH binary to typed messages. Owns byte-level concerns (offsets, endianness, message dispatch). Knows nothing about order books.
- `src/book/` — Order book state management. Receives parsed fields, never touches raw bytes. Tracks orders, maintains price levels, provides snapshots.
- `src/main.rs` — CLI replay tool. Wires parser and book together. Uses `clap` for args.
- Clean struct boundary between parser and book. The parser produces typed message structs/views; the book consumes them.
- Single crate with `lib` + `bin` targets. Not a workspace.

## Key Decisions
- Parser and book are modules in a single crate, not separate crates.
- Order book reconstruction is the headline, not the parser throughput.
- Each optimization is a separate commit with benchmark comparison.
- README documents the optimization journey with real data and flamegraphs.
- License: MIT + Apache-2.0 dual license.

## Project Management

This project uses **Taskwarrior** for task tracking (`project:tusk.*`). Use the custom skills:
- `/task-status` — progress report
- `/task-next` — what to work on next
- `/task-done` — complete current task

The `taskwarrior-pm` agent handles all backlog management. Before starting work, check `task next`. After finishing, mark `task done` and check what unblocked.
