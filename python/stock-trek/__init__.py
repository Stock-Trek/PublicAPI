"""Stock Trek time-series analysis.

A lightweight, composable time series and statistical toolkit.
"""

from . import data
from ._stock_trek import __version__ # type: ignore

__all__ = [
    "data",
]

__version__ = __version__
