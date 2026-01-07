author: Andreas
published: 2025-12-25 08:44:00
updated: 2026-01-07 15:07:00
topics: Static web site, Rust
title: Web site source code
snippet: This static website is generated with a custom cli based tool written in Rust called flow-web.

---

This static website is generated with a custom cli based tool written in [Rust](https://rust-lang.org/)
called [flow-web](https://codeberg.org/hfsoulz/flow-web.git).
`flow-web` is specifically written to generate this static website and is
available under the [GNU AGPL](https://gnu.org/licenses/agpl-3.0.html).

# Installing Rust

Install `Rust` from your package manager or by downloading from here:
[https://rust-lang.org/](https://rust-lang.org/).

# Getting the code

Install `git` from your package manager or by downloading from here:
[https://git-scm.com/install](https://git-scm.com/install).

Clone the `git` repository:

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
