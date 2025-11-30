clippy:
  cargo clippy -- -W clippy::nursery -W clippy::pedantic

build:
  wasm-pack build --release --target web
  rm pkg/README.md
  rm pkg/.gitignore
