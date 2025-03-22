# Rust Bucket | README.md

v0.1.0

*A simple file server using Rust.*

---

## Get started

*If needed, install Rust first*

`curl https://sh.rustup.rs -sSf | sh`

1. Clone the repo:

`git clone https://github.com/cursebreakers/rust_bucket.git`

2. Add files to the bucket folder:

*Example:* `cd rust_bucket && mkdir -p bucket/hello && touch bucket/hello/hello.md && echo "# Hello, World!" > bucket/hello/hello.md`

3. Compile and run w/cargo:

`cargo run`

Your bucket will be locally accessible via [http://localhost:1111/index](http://0.0.0.0:1111/index). It can then be accessed by any device connected to your local network at whatever the host device's local IP is. *(Use crtl+c to quit serving.)*

4. Have fun coding your bucket!

Extra pointers:
- Add a .env file with PORT variable to control port number


---

## Hosting your Bucket

There are numerous ways to deploy buckets for various use-cases, such as a simple file server, database, repository or a CDN. The following are merely a few of them.

**Link to "hosting with github and cloudflare workers"**

**Alternative Link to Railway**

**Alternative Link to "exposing my host with ngrok"**

*NOTE: I am not being paid to endorse any of these platforms.*

If you find a method that is not listed here, feel free to [let me know](mailto:hello@cursebreakers.net) and I may add it.

---

## File sharing

A bucket can share files to anyone with access to the bucket's address.

### Share a link to a file or directory**

Share a file or bucket URL using the links on the bucket page.

### Download a file or directory**

Download a file using the link on the bucket page.

*Coming soon: downloading a folder or a whole bucket*

*Coming soon: downloading files/folders/buckets with curl/wget/git*

---
 
## Styling your bucket

You can customize the appearance of your bucket by editing the index.html and style.css files in the bucket root. 

NOTE: If you wreck your templates beyond repair, you can redownload them from the [parent repo](https://github.com/cursebreakers/rust_bucket/tree/main/bucket), or use them as a reference to hopefully find your way back to style sanity.

---

# WORKING/NEXT:

*This is a planning section. These features are (likely) not yet implemented.*

**Share & Download Links**
- Share link opens device's share options or panel to copy link, etc
- Download entire folders
- Download entire buckets ("Dumping" a bucket)
  - using button/link
  - using curl and wget, etc
  - how does this relate/differ to git? Could some form of git implementation handle this better?

**Recursion Depth Control**
- Pass a variable to limit or expand the recursion depth of the bucket index

**Security & User Safety**
- How to secure the buckets?
  - Creating private/public bucket options
  - HTTPS/TLS/HSTS to encrypt packets/traffic to and from bucket
  - CORS?
- How and why/when/where to provision accounts and privelleges?

## FUTURE:

**User Accounts**
- email/username + password auth, OAuth via GitHub?
- API Keys for microservices and integrations
- allow auth via:
  - cli
    -  username/email & token (w/2fa optional)
  - GUI
    -  username/email & password (w/2fa optional)

**Git Integration**
- Build a bucket by connecting a git repo
- OAuth via GitHub?

**GUI**
- Client engine/logic using Rust & WASM only (no javascript, if possible)
- Let users add or remove bucket files using client GUI
- Control and Settings for User & Page Variables
  - User vars:
	- avatar
    - change email/password
    - private or public bucket
  - Page vars:
    - styles
	- bucket header
    - index recursion depth
- Gallery mode
  - client view that plays through bucket media
- Docs/Wiki folder
  - creates a docs/wiki section when added to bucket
  - users edit docs using markdown files
  - editor can toggle to raw md file edit

**Coolors integration**
- for randomzing/customizing bucket style palettes (using scss files?)

**Paid Hosting**
- Cursebreakers LLC will handle the hardware. Deploy a bucket on our infrastructure, with no git, CI/CD, or DNS configuration required.

---

# Credits & Acknowledgements 

## Author

Esau @ [Cursebreakers LLC](https://cursebreakers.net)

## Built with

- [Axum](https://crates.io/crates/axum) (`0.7`) - A web framework for building async APIs with Tokio.
- [Tokio](https://crates.io/crates/tokio) (`1`) - An asynchronous runtime for Rust, with full features enabled.
- [Tower HTTP](https://crates.io/crates/tower-http) (`0.5`) - Utilities for HTTP services, including file serving.
- [Tracing](https://crates.io/crates/tracing) (`0.1`) - Structured logging for Rust applications.
- [Tracing Subscriber](https://crates.io/crates/tracing-subscriber) (`0.3`) - A logging framework for `tracing`.

Honorable mention to [cargo-mommy](https://github.com/Gankra/cargo-mommy), for making programming with Rust much more fun.


