# rusting
> What is C++?

# About Rust
Rust is a multi-paradigm type/memory/thread-safe language with near-bare-metal performance.

Per [Simon Peyton Jones](https://www.youtube.com/watch?v=iSmkqocn0oQ), it sits right in that sweet spot of both being safe, fast and useful<sup>[citation needed]</sup> while supporting a myriad of paradigms, most notably OO and functional.

# Why should we hire you?
- Per-binding ownership and lifetimes.
- Reference counting by default to guarantee memory and thread-safety in one blow without any garbage-collection.
- Impeccable and standard tooling using `cargo`, with package management and building.
- A rich type-system borrowed from [haskell](https://www.haskell.org/) with compile-time checkable generics and polymorhpism using traits on types and methods.
- Stable ABI, but rapid development (new release every 6 weeks!).

# Getting started
There's the official book, [The Rust Programming Language](https://doc.rust-lang.org/stable/book/), an [awesome-rust](https://github.com/rust-unofficial/awesome-rust) page and this here repo you've somehow landed on.

Install rust either using the binaries provided in your Operating System or use `rustup` to manage multiple toolchains (think conda). 

If using `rustup` and in Linux:
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ rustup update
```
Testing the install:
```
$ rustc --version
```
Rust ships with `cargo`, so on to running some rust code.

Each directory in the repo root is a cargo project (created using `cargo new <project>`). Just move into one and `cargo run` to build and run a debug target of the lesson.

Benefit?