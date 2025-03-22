# Rust Bucket | README.md

v0.1.0

*A simple file server using Rust.*

---

## Get started

*If needed, install Rust first*

`curl https://sh.rustup.rs -sSf | sh`

### 1. Clone the repo:

`git clone https://github.com/cursebreakers/rust_bucket.git`

### 2. Add files to the bucket folder:

*Example:* `cd rust_bucket && mkdir -p bucket/hello && touch bucket/hello/hello.md && echo "# Hello, World!" > bucket/hello/hello.md`

*FILESIZE WARNING: Explicit memory/storage safety and rate limiting have not yet been developed and/or tested. I am not responsible for you maxing out your hardware limits or your VPS account by going crazy with the file sizes. Since this app currently stores files directly in the app container, only small files are recomended at this time. Future iterations will address this issue.*

*PRIVACY & SECURITY NOTE: This app exposes a port to your local network, and if deployed to an external web address like with a VPS, it can then be open to the world. For this reason, I recommend using wisdom and discretion when deciding on what types of files to include in your bucket. It is also recommended to stay up to date with network security and protocols to ensure that your self-hosting solution is safe.*

### 3. Compile and run w/cargo:

`cargo run`

Your bucket will be accessible at [http://localhost:1111/index](http://localhost:1111/index). It can also be accessed by any device connected to your local network at whatever the host device's local IP is. The server console logs should provide you with an address on startup.

Use **crtl+c** to quit serving.

*If you wish to put your bucket up on the world wide web, see the "Hosting your Bucket" section.*

### 4. Have fun coding your bucket!

Extra pointers:
- Add a .env file with PORT variable to control port number
- Edit the index.html and style.css to control the appearance of your bucket.
- use curl or wget to download your files from a bucket directly in your terminal

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

### Share a link to a file or directory

Share a file or bucket URL using the links on the bucket page.

### Download a file or directory

Download a file using the link on the bucket page.

*Coming soon: downloading a folder or a whole bucket*

*Coming soon: downloading files/folders/buckets with curl/wget/git*

---
 
## Styling your bucket

You can customize the appearance of your bucket by editing the index.html and style.css templates in the main bucket folder.

NOTE: 
- If you wreck your templates beyond repair, delete your index.html and one will be regenerated when you refresh your bucket index.
- You can redownload the templates from the [parent repo](https://github.com/cursebreakers/rust_bucket/tree/main/bucket), or use them as a reference to hopefully find your way back to style sanity.

## Hacking your bucket

This project is open source. You are encouraged to open up the files and make your bucket your very own. This includes **all aspects** of both server and client components.

To help get you oriented, these are the main elements of the program so far.

### The src folder

In the project repo you will find:
- the src folder with
  - main.rs
  - make_index.rs
  - make_style.rs *(coming soon)*

The main.rs file creates the server and routes (for now).

The make_index.rs file handles the generation of the bucket index and manipulation of the index.html when it is served.

The make_style.rs file (not ready yet) will control stylesheet generation, allowing variables to be passed to control bucket appearance.

### The bucket folder

- The bucket folder will contain an **index.html** and a linked **style.css** file by default. These templates are manipulated and regenerated (if necessary) by their make files in the **src** folder.

- Cloning a bucket clones that particular bucket's layout and styles, and may also contain files or folders from the person it was cloned from, unless those files were held back by the bucket owner (like with git ignore). 

- For example, if you cloned your bucket directly from me recently, you will have a *hello* folder and a *gifs* folder as well as a sweet rusty sunset stylesheet (feel free to edit/delete or keep these as you wish. Consider it a welcome gift).

### Philosophy

The main inspiration behind this project is to provide a simple yet powerful way to share files, without limiting versatility of the engine or the creativity of the user/developer. This means that throughout the design of the UI and core app engine, the intent is creating a modular and customizable framework to not only store and serve files, but also for use in larger ecosystems for content delivery or git repositories, and even act as a standalone website.

---

# WORKING/NEXT:

This is the planning section. These features are (99% likely) not yet implemented, nor may they ever be. Please [let me know](mailto:hello@cursebreakers.net) if you would like to request a feature.

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

**Application Databases**
- To store and control network data, variables, structures, logic, etc

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

**Docker Integration**
- Docker install/setup instructions
- Build a bucket with a docker container
- OAuth via Docker?

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

**Email servers & client**
- If we get this far.

**Paid hosting**
- Cursebreakers LLC will handle the hardware. Deploy a bucket on our infrastructure, with no git, CI/CD, or DNS configuration required by users.

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


