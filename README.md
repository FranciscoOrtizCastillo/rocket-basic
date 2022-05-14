# Rocket, Framework Backend de Rust (Básico)

```
rustc --version
rustup update

cargo --version

# Create a new project
cargo new rocket-basic

cd rocket-basic
cargo run

```

# Docs

https://rocket.rs

https://rocket.rs/v0.5-rc/guide/getting-started/#installing-rust


Add rocket = "0.5.0-rc.2" to Cargo.toml



https://api.rocket.rs/v0.5-rc/rocket_dyn_templates/index.html

[dependencies.rocket_dyn_templates]
version = "0.1.0-rc.2"
features = ["handlebars", "tera"]

Uso solo handlebars