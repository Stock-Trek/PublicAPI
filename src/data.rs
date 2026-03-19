use rust_decimal::Decimal;

pub type Timestamp = i64;

#[derive(Debug, Clone, Copy)]
pub enum Real {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Decimal(Decimal),
}

#[derive(Debug, Clone)]
pub struct Datum {
    timestamp: Timestamp,
    value: Real,
}

impl Datum {
    pub fn new(timestamp: Timestamp, value: Real) -> Self {
        Self { timestamp, value }
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    pub fn value(&self) -> Real {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct TimeSeries {
    data: Vec<Datum>,
}

impl TimeSeries {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn add_datum(&mut self, datum: Datum) {
        self.data.push(datum);
    }

    pub fn get_datum(&self, index: usize) -> Option<&Datum> {
        self.data.get(index)
    }

    pub fn data(&self) -> &[Datum] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for TimeSeries {
    fn default() -> Self {
        Self::new()
    }
}
