# build-your-own-jira-with-rust
A test-driven workshop to learn Rust building your own JIRA clone!

## First steps

### Building the project

You can build it running:
```bash
cargo build
```

Use
```bash
cargo build --release
```
to generate an optimised binary.

### Testing the project

You can build it running:
```bash
cargo test
```

### Documentation

```bash
# We rely on the nightly compiler for automatic semantic link generation
cargo +nightly doc --open
```