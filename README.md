# Build your own JIRA with Rust (Rust London Code Dojo)

It's out pleasure to welcome you to the Rust London Code Dojo!

You will be working through a series of test-driven exercises, or koans, to learn Rust while building your own JIRA clone!

You can get started with
```bash
git clone git@github.com:LukeMathWalker/build-your-own-jira-with-rust.git
cargo install -f --path koans-framework 
git checkout -b my-solution
koans --path jira-wip
```

Follow the instructions shown in the terminal to get started with the first koan.

Run this command from the top-level folder
```bash
koans --path jira-wip
```
to verify your current solutions and move forward in the workshop.

Enjoy!

## Requirements 

### Software 

- **Install Rust** (follow instructions [here](https://www.rust-lang.org/tools/install))  
  If Rust is already installed on your system, make sure you are running on the latest compiler version, `1.41` (`cargo --version`).  
  If not, update using `rustup update` (or another appropriate command depending on how you installed Rust on your system).

- **Install our `koans` CLI**: you will need it to work through the exercises.  
  You can install `koans` system-wide from the top-level directory using:
  ```bash
  cargo install -f --path koans-framework 
  ```
  You can verify that everything is working properly running the command:
  ```bash
  koans --help
  ```
  
### Knowledge

This workshop is designed for people who have experience using other programming languages and are just getting 
started with Rust.

If you run into any issue with the assumed level of Rust knowledge, please ping me and we'll sort it together!

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
