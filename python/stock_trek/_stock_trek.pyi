"""Stub file for the compiled Rust module."""

class Datum:
    """A single data point in a time series.
    
    Args:
        timestamp: Unix timestamp in seconds
        value: The value at this timestamp (can be int, float, or decimal string)
    """
    def __init__(self, timestamp: int, value: float) -> None: ...
    def timestamp(self) -> int:
        """Return the timestamp of this datum."""
        ...
    def value(self) -> float:
        """Return the value of this datum."""
        ...
    def __repr__(self) -> str: ...

class TimeSeries:
    """A time-ordered collection of data points.
    
    Provides efficient storage and statistical operations on time series data.
    """
    def __init__(self) -> None: ...
    
    @staticmethod
    def with_capacity(capacity: int) -> TimeSeries:
        """Create a new TimeSeries with pre-allocated capacity.
        
        Args:
            capacity: Initial capacity for the underlying storage
        """
        ...
    
    def add_datum(self, datum: Datum) -> None:
        """Add a datum to the time series.
        
        Data points are maintained in timestamp order.
        
        Args:
            datum: The data point to add
        """
        ...
    
    def get_datum(self, index: int) -> Datum:
        """Get the datum at the specified index.
        
        Args:
            index: Position in the time series (0-based)
            
        Returns:
            The datum at that position
        """
        ...
    
    def data(self) -> list[Datum]:
        """Return a copy of all data points in the series."""
        ...
    
    def len(self) -> int:
        """Return the number of data points in the series."""
        ...
    
    def is_empty(self) -> bool:
        """Return True if the series contains no data points."""
        ...
