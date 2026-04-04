# stock-trek

A lightweight, composable time series and statistical toolkit designed for running crypto bots on [stock-trek.com](https://stock-trek.com). Rust-native core with optional Python bindings

## Overview

stock-trek provides core abstractions and utilities for working with market data, including:

- Order books
- Aligned/Rolling windows
- Ticks
- Statistical and analytical functions

## Installation

Add to your Cargo.toml:

```rs
[dependencies]
stock-trek = "0.2.4"
```

## Python Bindings

stock-trek also provides Python bindings which can be installed via

`pip install stock-trek`

## Usage

Implement the ```StockTrekAlgorithm``` trait and register it with the annotation `#[register_algorithm]`:

```rs
use stock_trek::prelude::*;
use stock_trek::signal::*;

pub struct MyAlgo;

#[register_algorithm]
impl StockTrekAlgorithm for MyAlgo {
    fn create_signal(&self, context: StockTrekContext) -> StockTrekSignal {
      ...
    }
}
```

Stock-Trek verifies code before running it and disallows certain syntax elements. To verify code locally, install it with

```sh
cargo install stock-trek
```

then run the verify command with

```sh
stock-trek verify --file ./path/to/my/code/file.rs
```

## Roadmap

Planned features include:

- Technical indicators (EMA, RSI, MACD, etc.)
- Backtesting and simulation utilities

## Status

This project is in early development (0.x). APIs may change.

## License

MIT
