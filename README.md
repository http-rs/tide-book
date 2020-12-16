# Tide book
This repository contains the source for the [Tide Book], the book is written in the `markdown` format and is built using [mdbook].

- [简体中文](https://github.com/zzy/tide-zh-cn), [在线阅读](https://tide.budshome.com)

## Development
The documentation can be edited using any text editor. Most commonly used editors support syntax highlighting for the `markdown` format. To view your changes you can install the [mdbook] tool locally, assuming you already have a working `Rust` setup;
```console
> cargo install mdbook
```

With [mdbook] installed you can use it to build and serve the documentation on your local system;
```console
> mdbook serve
```
this will start a local server that will be available on [localhost](http://localhost:3000) and will automatically build and re-build the documentation when it changes.

## Contributing
Want to help out? Check out our [The "Contributing" document][contributing]

### Conduct

The Tide project adheres to the [Contributor Covenant Code of
Conduct](https://github.com/http-rs/tide/blob/main/.github/CODE_OF_CONDUCT.md).
This describes the minimum behavior expected from all contributors.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[Tide Book]: https://http-rs.github.io/tide-book/
[mdbook]: https://rust-lang.github.io/mdBook/
[contributing]: https://github.com/http-rs/tide/blob/main/.github/CONTRIBUTING.md
