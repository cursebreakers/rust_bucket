# Rust Bucket | REAMDE.md

*v0.1.0*

A simple file server using Rust.

---

## Get started

Clone the repo:

`git clone https://github.com/cursebreakers/rust_bucket.git`

Add files to the bucket folder:

`mkdir -p bucket/hello && touch bucket/hello/hello.md && echo "# Hello, World!" > bucket/hello/hello.md`

Compile and run w/cargo:

`cd rust_bucket && cargo run`

Your bucket will be accessible via [http://0.0.0.0:1111/index](http://0.0.0.0:1111/index). *(Use crtl+c to quit serving.)*

---
# Credits & Acknowledgements 

## Built With

This project leverages the following Rust libraries:

- [Axum](https://crates.io/crates/axum) (`0.7`) - A web framework for building async APIs with Tokio.
- [Tokio](https://crates.io/crates/tokio) (`1`) - An asynchronous runtime for Rust, with full features enabled.
- [Tower HTTP](https://crates.io/crates/tower-http) (`0.5`) - Utilities for HTTP services, including file serving.
- [Tracing](https://crates.io/crates/tracing) (`0.1`) - Structured logging for Rust applications.
- [Tracing Subscriber](https://crates.io/crates/tracing-subscriber) (`0.3`) - A logging framework for `tracing`.

## Author

Esau @ [Cursebreakers LLC](https://cursebreakers.net)


