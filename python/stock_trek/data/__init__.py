"""Data structures for time-series analysis."""

import importlib

__all__ = ["Datum", "TimeSeries"]

def __getattr__(name):
    if name in __all__:
        # Get the _stock_trek module
        core = importlib.import_module(".._stock_trek", __package__)
        # Access its data attribute
        data_mod = getattr(core, "data")
        return getattr(data_mod, name)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")

def __dir__():
    return __all__
