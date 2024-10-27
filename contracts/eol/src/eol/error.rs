use cosmwasm_std::Timestamp;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EOLError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("Time is yet to be out of bounds {out_of_bounds_limit}")]
    TimeInBoundsError { out_of_bounds_limit: Timestamp },

    #[error("invalid request")]
    InvalidRequest {},
}

pub type EOLResult<T> = Result<T, EOLError>;

