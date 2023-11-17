# moc3 [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/moc3.svg
[crates.io]: https://crates.io/crates/moc3

## What is it?

A binary loader for `.moc3` files. It has been tested and is able to load the
following versions of this data format:

- v3.0
- v3.3
- v4.0 (though this version did not seem to be any different than v3.3)
- v4.2
- v5.0

It can correctly identify all sections of these versions, and load all the
relevant data into meaningful data types like `ArtMesh`, `Part`, `Deformer`, etc.

## How do I obtain this majestic tool?

Run the following Cargo command in your project directory (assuming you have [cargo-edit](https://github.com/killercup/cargo-edit) installed):

```fish
cargo add moc3
```

Or add the following line to your `Cargo.toml` (in the `[dependencies]` array):

```toml
moc3 = "^ 0.2"
```

## How do I use it?

```rust
use moc3::Model;
use std::io::Read;

fn main() {
  let moc3 = std::fs::read("./path/to/some.moc3").unwrap();
  let model = Model::read(moc3).unwrap();
  println!("{model:#?}");
}
```

## How was this made?

- Carefully, without using or referencing any code or libraries from the format vendor.
- The [ImHex](https://github.com/WerWolv/ImHex) highlighting patterns from the [MOC3ingbird Exploit](https://github.com/OpenL2D/moc3ingbird) (CVE-2023-27566) was instrumental in understanding this format.
- The discovery process for undocumented binary formats is described [here](https://gist.github.com/colstrom/f671d1583662de47b505a42a75b3a44b).

## License

`moc3` is available under the MIT License. See `LICENSE.txt` for the full text.

While the license is short, it's still written in fancy lawyer-speak. If you
prefer more down-to-earth language, consider the following:

- tl;drLegal has a simple visual summary available [here](https://www.tldrlegal.com/license/mit-license).
- FOSSA has a more in-depth overview available [here](https://fossa.com/blog/open-source-licenses-101-mit-license/).
