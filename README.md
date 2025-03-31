# kbo-gui
Graphical user interface for [kbo](https://github.com/tmaklin/kbo) built with [Dioxus](https://dioxuslabs.com/).

Try it out at [https://maklin.fi/kbo](https://maklin.fi/kbo).

## Usage
See the [kbo documentation](https://docs.rs/kbo).

## Install
### Build from source
#### Dependencies
- [dioxus-cli](https://docs.rs/dioxus-cli) v0.6
- Rust >= v1.80.0 (stable)

#### Compiling
Compile from source with
```
git clone https://github.com/tmaklin/kbo-gui
cd kbo-gui
dx build --release --platform web

```

#### Deploying to web
After compiling, create a distributable .tar.gz file with
```
cp -rf target/dx/kbo-gui/release/web/public ./kbo
tar -zcvf kbo.tar.gz kbo
```

Then extract `kbo.tar.gz` to a folder that can be served by a web server. The GUI
will be available at `https://<web server base path>/kbo`.

If you need to change the relative `/kbo` path, modify `web.app.base_path` in
Dioxus.toml.

#### Deploying to other platforms
See the Dioxus [guide on
bundling](https://dioxuslabs.com/learn/0.6/guide/bundle#) to build for other
platforms.

Only the `--platform web` option is currently supported.

## Development
Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload true
```

- Open the browser to http://localhost:8080

## License
kbo-gui is dual-licensed under the [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE) licenses.
