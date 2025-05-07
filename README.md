# Usage

`Defensive Programming Pattern` to allow to check for conditions without panicking in production code (i.e. not in tests).

Add to **Cargo.toml**:
```
rust-soft-assert = { git = "https://github.com/grooviegermanikus/rust-soft-assert.git", tag = "v0.2" }
```

```rust
soft_assert!(false, "This is a soft assert");
```
