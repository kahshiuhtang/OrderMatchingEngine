# Order Matching Engine

## About

Rust-based implementation of a order matching engine. Purely out of self-interest 

## Installation

Use Cargo to install trunk. Also make sure the WASM build target is available

```bash
$ cargo install trunk

$ rustup target add wasm32-unknown-unknown

```
## Run
```bash

$ cargo watch -c -q -x run

$ trunk server --open --port 8081

```

or 
```bash

$ cargo run
$ trunk serve

```

### Options
[-h] [-v] [-n | -l] [s]