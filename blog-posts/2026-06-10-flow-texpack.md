author: Andreas
published: 2026-06-10 14:01:00
updated: 2026-06-10 14:01:00
topics: flow-texpack, Texture Atlas, Image Atlas, generator, Rust
title: flow-texpack: A program that will allow you to generate texture atlas.
snippet: flow-texpack is a program that will allow you to generate texture atlas from input images (BMP, HDR, JPG, PNG, TGA, TIFF, WEBP).

---

[flow-texpack](https://git.luflow.net/hfsoulz/flow-texpack.git) is a program that
will allow you to generate texture atlas from input images (BMP, HDR, JPG, PNG,
TGA, TIFF, WEBP). The application generates both texture atlas and descriptions
file that can be read by a game.

### Installing Rust

Install `Rust` from your package manager or by downloading from here:
[https://rust-lang.org/](https://rust-lang.org/).

### Getting the code

Install `git` from your package manager or by downloading from here:
[https://git-scm.com/install](https://git-scm.com/install). The [git
repository](https://git.luflow.net/hfsoulz/flow-texpack.git) can also be browsed
online.

Clone the `git` repository:

```sh
git clone https://git.luflow.net/hfsoulz/flow-texpack.git
```

### Compiling the code

Build using release mode and install locally:

```sh
cargo install --locked --path .
```

The binary produced should be located here: `~/.cargo/bin/flow-texpack` (on
GNU/Linux). Make sure `~/.cargo/bin` is in your `PATH`.

### Usage

Show available options:

```sh
flow-texpack -h
```

or

```sh
flow-texpack --help
```

### Examples

Generate from input `data/characters` and `data/tiles`, write output to
`out/atlas` and enable the options: `premultiply` pixels by their alpha
channel, `trim` excess transparency off the textures, `remove duplicate
textures` from the atlas, enable `rotation` of textures 90 degrees clockwise,
`pad` each texture by 2 pixels and finally enable `verbose` output mode.

```sh
flow-texpack -i data/characters data/tiles -o out/atlas -m -t -u -r -p 2 -v
```

Enable `load filter` so that only `TGA` images are included in the texture atlas:

```sh
flow-texpack -i data/tiles -o out/atlas --load-filter tga -v
```

Enable rect heuristic `AreaFit`:

```sh
flow-texpack -i data/tiles -o out/atlas --rect-heuristic area-fit -v
```

Enable output `atlas size` of **2048x2048**:

```sh
flow-texpack -i data/tiles -o out/atlas --atlas-size pot2048 -v
```

Read input files/directories from `input.txt` but exclude all in `exclude.txt`:

```sh
flow-texpack --input-file input.txt --exlude-file exclude.txt -o out/atlas -v
```

`Adjust atlas size` automatically so that texture will fit:

```sh
flow-texpack -i data/characters -o out/atlas --adjust-size -v
```

`Adjust texture size` so that it will fit given atlas size:

```sh
flow-texpack -i data/characters -o out/atlas --adjust-fit -v
```

### License

[flow-texpack](https://git.luflow.net/hfsoulz/flow-texpack.git) is licensed under the
zlib license. This license allows you to use `flow-texpack` freely in any software.
