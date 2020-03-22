# Build your own JIRA with Rust (Rust London Code Dojo)

It's out pleasure to welcome you to the Rust London Code Dojo!

You will be working through a series of test-driven exercises, or koans, to learn Rust while building your own JIRA clone!

This workshop is designed for people who have experience using other programming languages and are just getting
started with Rust.  
If you run into any issue with the assumed level of Rust knowledge, please ping us and we'll sort it together!

## Requirements

- **Rust** (follow instructions [here](https://www.rust-lang.org/tools/install)).  
    If Rust is already installed on your system, make sure you are running on the latest compiler version, `1.42` (`cargo --version`).  
    If not, update using `rustup update` (or another appropriate command depending on how you installed Rust on your system).
- _(Optional)_ An IDE with Rust autocompletion support. 
    We recommend one of the following:
    - [IntelliJ IDEA](https://www.jetbrains.com/idea/) with the [`intellij-rust`](https://intellij-rust.github.io) plugin;
    - [Visual Studio Code](https://code.visualstudio.com) with the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.  
        Checkout the [`rust-in-peace`](https://marketplace.visualstudio.com/items?itemName=gilescope.rust-in-peace) extension for a battery-included Rust setup on VS Code.
  
## Getting started 

```bash
git clone git@github.com:LukeMathWalker/build-your-own-jira-with-rust.git
cd build-your-own-jira-with-rust

# Our `koans` CLI, you will need it to work through the exercises. 
# You can run `koans --help` to check that everything is running properly
cargo install -f --path koans-framework

# Work on your solution in a branch. 
git checkout -b my-solution

# Get started!
koans --path jira-wip
```

Follow the instructions shown in the terminal to get started with the first koan.

Run this command from the top-level folder
```bash
koans --path jira-wip
```
to verify your current solutions and move forward in the workshop.

Enjoy!

## References

Throughout the workshop, the following resources might turn out to be useful:

* The [Rust Book](https://doc.rust-lang.org/book/);
* The [Rust documentation](https://doc.rust-lang.org/std/) (you can also open the documentation offline with `rustup doc`!).


## Solutions

Under `jira-cli`, you can find a worked-out solution.

You can build it running:
```bash
cargo build --bin jira-cli
```

You can try it out running:
```bash
cargo run --bin jira-cli -- --help
```

You can run its tests running:
```bash
cargo test --bin jira-cli
```

You can browse its documentation with:
```bash
# We rely on the nightly compiler for automatic semantic link generation
cargo +nightly doc --manifest-path jira-cli/Cargo.toml --open
```
