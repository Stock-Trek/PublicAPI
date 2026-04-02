use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Error, Debug)]
#[repr(u8)]
pub enum StockTrekError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    #[error("Stats error: {0}")]
    Stats(#[from] StatsError),
}

#[derive(Error, Debug)]
#[repr(u8)]
pub enum ValidationError {
    #[error("{} must be not be empty", name)]
    IsEmpty { name: String },
    #[error("{} must exist in {}", name, container)]
    DoesNotExist { name: String, container: String },
    #[error("{} must not exist in {}", name, container)]
    AlreadyExists { name: String, container: String },
    #[error("{} must be positive but is {}", name, value)]
    IsNotPositive { name: String, value: Decimal },
    #[error("{} must be zero or positive but is negative {}", name, value)]
    IsNegative { name: String, value: Decimal },
    #[error(
        "{} has value {} which is not a multiple of {} ",
        name,
        value,
        increment
    )]
    InvalidIncrement {
        name: String,
        value: Decimal,
        increment: Decimal,
    },
}

#[derive(Error, Debug)]
#[repr(u8)]
pub enum StatsError {
    #[error("")]
    DivisionByZero,
    #[error("")]
    EmptyInput,
    #[error("")]
    MismatchedLengths,
    #[error("")]
    InsufficientDegreesOfFreedom,
    #[error("")]
    DomainError { message: String },
    #[error("")]
    InvalidLag,
    #[error("")]
    InvalidParameters,
    #[error("")]
    ZeroVariance,
}
