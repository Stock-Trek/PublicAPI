"""Stock Trek time-series analysis."""

from . import data
from ._stock_trek import __version__

__all__ = [
    "data",
]

__version__: str
