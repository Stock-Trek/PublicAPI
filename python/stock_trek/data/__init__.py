"""Data structures for time-series analysis."""

from .._stock_trek.data import Datum, TimeSeries # type: ignore

__all__ = [
    "Datum",
    "TimeSeries",
]
