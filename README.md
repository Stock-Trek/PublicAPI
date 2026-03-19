# stock-trek

A lightweight, composable time series and statistical toolkit designed for running crypto bots on [stock-trek.com](https://stock-trek.com). Rust-native core with optional Python bindings

## Overview

stock-trek provides core abstractions and utilities for working with price data, including:

- Strongly-typed Datum representation
- Efficient TimeSeries container
- Statistical and analytical functions
- Functional-style transformations and mappings

It is built with a focus on:

- Performance (zero-cost abstractions where possible)
- Ergonomics (clean, composable APIs)
- Extensibility (designed to grow into simulation/backtesting)

## Installation

Add to your Cargo.toml:

```rs
[dependencies]
stock-trek = "0.1.0"
```

## Python Bindings

stock-trek also provides Python bindings which can be installed via

`pip install stock-trek`

## Roadmap

Planned features include:

- Technical indicators (EMA, RSI, MACD, etc.)
- Backtesting and simulation utilities
- OHLCV-native data structures

## Status

This project is in early development (0.x). APIs may change.

## License

MIT
