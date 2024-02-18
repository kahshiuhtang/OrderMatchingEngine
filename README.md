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
$ cargo build

$ ./target/debug/order_matching_engine

$ trunk serve

```

or 
```bash

$ cargo run
$ trunk serve

```

### Options
[-h] [-v] [-n | -l] [s]