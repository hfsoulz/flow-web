# About flow-web

flow-web is a custom cli tool written in Rust. flow-web is used to generate the
static website at [https://www.luflow.net](https://www.luflow.net).

# Installation

Install Rust from your package manager or by downloading from here:
[https://rust-lang.org/](https://rust-lang.org/).

# Generate the website

Build using release mode:

```sh
cargo build --release
```

Run to generate site:

```sh
cargo run --release
```

The generated output can be found in the '**output**' folder.

# Serve locally

Run the following command to serve locally using **servez** as an example:

```sh
servez output
```

Then, visit the following url in a web browser:

http://localhost:8080/

You can stop the server pressing CTRL+c.

# LICENSE

See the file 'LICENSE' for license information.
