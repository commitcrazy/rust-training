# Rust Training

Practice projects from the [Microsoft Rust Training](https://github.com/microsoft/RustTraining) course — a hands-on guide to learning Rust for developers coming from other languages.

Each chapter's exercises are stored as separate Cargo projects in this repo.

## Projects

| Project | Chapter | Topics |
|---------|---------|--------|
| [hello_rust](./hello_rust) | [Ch 2 — Getting Started](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch02-getting-started.md) | Variables, mutability, loops, pattern matching |
| [temperature_converter](./temperature_converter) | [Ch 3 — Built-in Types and Variables](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch03-built-in-types-and-variables.md) | Floating-point types (f64), functions, match guards, &'static str, string formatting |
| [fizzbuzz_with_expressions](./fizzbuzz_with_expressions) | [Ch 4 — Control Flow](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch04-control-flow.md) | For loops, range expressions, pattern matching (match), tuple destructuring, wildcard patterns |
| [word_frequency_counter](./word_frequency_counter) | [Ch 5 — Data Structures and Collections](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch05-data-structures-and-collections.md) | HashMap, entry API, iterators, split_whitespace, String vs &str, mutable references |
| [shape_area_calculator](./shape_area_calculator) | [Ch 6 — Enums and Pattern Matching](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch06-enums-and-pattern-matching.md) | Enums with struct-like variants, impl blocks, methods, match pattern matching, variant destructuring, std::f64::consts::PI |
| [spot_the_borrow_checker_error](./spot_the_borrow_checker_error) | [Ch 7 — Ownership and Borrowing](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch07-ownership-and-borrowing.md) | Ownership, immutable/mutable borrows, references, non-lexical lifetimes (NLL), &str vs String, move semantics |
| [module_visibility](./module_visibility) | [Ch 8 — Crates and Modules](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch08-crates-and-modules.md) | Modules (`mod`), visibility (`pub`), nested submodules, `super::` paths, path resolution (`::`), private-by-default encapsulation |
| [parse_config_value](./parse_config_value) | [Ch 9 — Error Handling](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch09-error-handling.md) | `Result<T, E>`, custom error enums, `thiserror` crate, `#[derive(Error)]`, `#[from]` auto-conversion, `?` operator, error variants (unit/tuple/struct) |
| [generic_summary_trait](./generic_summary_trait) | [Ch 10 — Traits and Generics](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch10-traits-and-generics.md) | Trait definition, `impl Trait for Type`, trait methods, `impl Trait` parameter syntax, static dispatch, polymorphism, `format!` macro |
| [temperature_conversion_library](./temperature_conversion_library) | [Ch 11 — From and Into Traits](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch11-from-and-into-traits.md) | `From` trait, `Into` trait, `TryFrom` trait, infallible vs. fallible conversions, `Display` trait, custom error messages |
| [derive_and_custom_debug](./derive_and_custom_debug) | [Ch 12 — Closures and Iterators](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch12-closures-and-iterators.md) | `#[derive]` macro (`Clone`, `PartialEq`), manual `impl Debug`, `Formatter::debug_struct` builder, field redaction for sensitive data, `std::fmt` module |
| [thread_safe_counter](./thread_safe_counter) | [Ch 13 — Concurrency](https://github.com/microsoft/RustTraining/blob/main/python-book/src/ch13-concurrency.md) | `Arc<Mutex<T>>` shared state, `tokio` async runtime, `#[tokio::main]`, `tokio::spawn`, async closures, `futures::join_all`, thread-safe counter pattern |

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (rustc, cargo)

## Running a project

```sh
cd hello_rust
cargo run
```
