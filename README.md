# kbo-gui
WIP Graphical user user interface for [kbo](https://github.com/tmaklin/kbo) built with [Dioxus](https://dioxuslabs.com/).

## Development
Run the following command in the root of the project to start the Dioxus dev server:

```bash
cargo patch-crate
dx serve --hot-reload
```

- Open the browser to http://localhost:8080

## Build from source
### Dependencies
- [dioxus-cli](https://docs.rs/dioxus-cli)
- [patch-crate](https://docs.rs/patch-crate)
- Rust >= v1.80.0 (stable)

### Building
```
cargo patch-crate
dx build --release
```

## License
kbo-gui is dual-licensed under the [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE) licenses.
