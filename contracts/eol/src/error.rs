use thiserror::Error;

use cosmwasm_std::{CoinsError, StdError, Timestamp};

use crate::{
    authenticator::AuthenticatorError,
};

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CoinsError(#[from] CoinsError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid denom: {denom}")]
    InvalidDenom { denom: String },

    #[error("Duplicated denom: {denom}")]
    DuplicatedDenom { denom: String },

    #[error("Current time {current} not within time limit {start:?} - {end}")]
    NotWithinTimeLimit {
        current: Timestamp,
        start: Option<Timestamp>,
        end: Timestamp,
    },

    #[error("Authenticator error: {0}")]
    AuthenticatorError(#[from] AuthenticatorError),

    #[error("Requested entry not found")]
    NotFound {},
}
