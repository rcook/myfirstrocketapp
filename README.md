# MyFirstRocketApp

## Prerequisites

### Cargo

Follow Cargo [installation instructions][cargo-installation] to install Cargo using rustup.

### cargo-watch

```bash
cargo install cargo-watch
```

### Build and test

```bash
make
```

### How I built this

[Getting Started][rocket-getting-started]

### Running locally

You'll need to create a `Rocket.toml` file in the root of this project:

```bash
cp Rocket.toml.template Rocket.toml
```

Then edit the `url` field under `sqlite3` to point to a path at which the app will create your SQLite3 database.

## Licence

[MIT License][licence]

[cargo-installation]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[licence]: LICENSE
[rocket-getting-started]: https://rocket.rs/v0.4/guide/getting-started/
