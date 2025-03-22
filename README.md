# Rust Bucket | README.md

v0.1.0

*A simple file server using Rust.*

---

## Get started

1. Clone the repo:

`git clone https://github.com/cursebreakers/rust_bucket.git`

2. Add files to the bucket folder:

`cd rust_bucket && mkdir -p bucket/hello && touch bucket/hello/hello.md && echo "# Hello, World!" > bucket/hello/hello.md`

3. Compile and run w/cargo:

`cargo run`

Your bucket will be accessible via [http://localhost:1111/index](http://0.0.0.0:1111/index). It will also be accessible by any device connected to your local network at whatever your device's local IP is. *(Use crtl+c to quit serving.)*

## File Sharing

By when hosting your bucket you can share files to anyone with access to the bucket's address.

*Link to "hosting the rust bucket server"*

**Download a file or directory**

**Share a link to a file or directory**

## Styling your bucket

You can customize the appearance of your bucket by editing the index.html and style.css files in the bucket root. 

NOTE: If you wreck your templates beyond repair, you will likely need to redownload them and replace them from the parent repo, or you can use them as a reference to find your way back.

---
# WORKING

# NEXT:

## Future:

**Share & Download Links**
- Share link opens device's share options or panel to copy link, etc
- Download links for folders and entire buckets (git clone?)

**User Accounts**
- email + password auth, OAuth?
- API Keys for microservices and integrations

**UI**
- Control User & Page Variables
- Gallery mode
- folder sync/file upload menu
- Docs/Wiki folder w/ Markdown rendering/editing

**Git Integration**
- build a bucket from by connecting a git repo
- OAuth via GitHub?

**Coolors.io integration**
- for randomzing/customizing style palettes (using scss files?)

**Remote Hosting**
- Deploy a bucket on Cursebreakers LLC Infrastructure

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


