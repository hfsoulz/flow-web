author: Andreas
published: 2025-12-25 08:44:00
updated: 2025-12-25 08:44:00
topics: Static web site, Rust
title: Web site source code
snippet: This static website is generated with a custom cli based tool written in Rust called flow-web.

---

This static website is generated with a custom cli based tool written in [Rust](https://rust-lang.org/)
called [flow-web](https://codeberg.org/hfsoulz/flow-web.git).
[flow-web](https://codeberg.org/hfsoulz/flow-web.git) is specifically written
to generate this static website and is available under the [GNU
AGPL](https://gnu.org/licenses/agpl-3.0.html).

# Installing Rust

Install Rust from your package manager or by downloading from here:
[https://rust-lang.org/](https://rust-lang.org/).

Check rustc version:

```sh
rustc --version
rustc 1.92.0 (ded5c06cf 2025-12-08) (Arch Linux rust 1:1.92.0-1)
```

Check cargo version:

```sh
cargo --version
cargo 1.92.0 (344c4567c 2025-10-21) (Arch Linux rust 1:1.92.0-1)
```

# Getting the code

Install git from your package manager or by downloading from here:
[https://git-scm.com/install](https://git-scm.com/install).

Check git version:

```sh
git --version
git version 2.38.1
```

Clone the git repository:

```sh
git clone https://codeberg.org/hfsoulz/flow-web.git
```

# Compiling the code

cd into the cloned dir:

```sh
cd flow-web
```

Build using release mode:

```sh
cargo build --release
```

Generate the site using release mode:

```sh
cargo run --release
```

The generated output can be found in '**output**' folder.

# Serve locally

Run the following command to serve locally using **[servez](https://www.npmjs.com/package/servez)** as an example:

```sh
servez output
```

Then, visit the following url in a web browser:

http://localhost:8080/

You can stop the server pressing CTRL+c.

servez can be installed through [Node.js](https://nodejs.org/en) like so:

```sh
npm install -g servez
```
