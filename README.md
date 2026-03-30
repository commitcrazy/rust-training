# Rust Training

Practice projects from the [Microsoft Rust Training](https://github.com/microsoft/RustTraining) course — a hands-on guide to learning Rust for developers coming from other languages.

Each chapter's exercises are stored as separate Cargo projects in this repo.

## Projects

| Project | Chapter | Topics |
|---------|---------|--------|
| [hello_rust](./hello_rust) | [Ch 2 — Getting Started](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch02-getting-started.md) | Variables, mutability, loops, pattern matching |
| [temperature_converter](./temperature_converter) | [Ch 3 — Built-in Types and Variables](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch03-built-in-types-and-variables.md) | Floating-point types (f64), functions, match guards, &'static str, string formatting |
| [fizzbuzz_with_expressions](./fizzbuzz_with_expressions) | [Ch 4 — Control Flow](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch04-control-flow.md) | For loops, range expressions, pattern matching (match), tuple destructuring, wildcard patterns |

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (rustc, cargo)

## Running a project

```sh
cd hello_rust
cargo run
```
