use crate::{
    eol::error::EOLError,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Coins, Timestamp, Uint128};
use EOLError::TimeInBoundsError;

use super::error::EOLResult;

/// State for tracking EOL
#[cw_serde]
#[derive(Default)]
pub struct EOL {
    /// Amount of time user if inactive can this account be used
    pub inactivity_time_period: Timestamp,

    /// The last time the account spent
    /// This is used to check if we are in a new period
    pub last_spent_at: Timestamp,

}

impl EOL {
    pub fn new(inactivity_period: Timestamp, last_spent: Timestamp) -> Self {
        Self {
            inactivity_time_period: inactivity_period,
            last_spent_at: last_spent, // should be block.Time at initiation
        }
    }

    pub fn update(
        &mut self,
        last_spent_at: Timestamp,
    ) -> &mut Self {
        self.last_spent_at = last_spent_at;

        self
    }

    /// ensure that the value spent in the period is not over the limit
    pub fn ensure_out_of_limit(&self, curr_time: Timestamp) -> EOLResult<()> {
        let eol_time = self.last_spent_at.plus_seconds(self.inactivity_time_period.seconds());
        if curr_time.le(&eol_time.clone()) {
            Err(TimeInBoundsError {
                out_of_bounds_limit: eol_time,
            })
        } else {
            Ok(())
        }
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::rstest;
//
//     #[rstest]
//     #[case::no_delta(vec![], vec![], vec![])]
//     #[case::no_delta(vec![Coin::new(100, "uosmo")], balances_before_spent.clone(), vec![])]
//     #[case::no_delta(vec![Coin::new(100, "uosmo"), Coin::new(1023, "usomething")], balances_before_spent.clone(), vec![]
//     )]
//     #[case::receive(
//         vec![Coin::new(100, "uosmo")],
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![]
//     )]
//     #[case::receive(
//         vec![Coin::new(100, "uosmo")],
//         vec![Coin::new(101, "uosmo"), Coin::new(200, "usomething")],
//         vec![]
//     )]
//     #[case::spend(
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(99, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(1, "uosmo")]
//     )]
//     #[case::spend(
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(99, "uosmo"), Coin::new(199, "usomething")],
//         vec![
//             Coin::new(1, "uosmo"),
//             Coin::new(1, "usomething"),
//         ]
//     )]
//     #[case::spend(
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(99, "uosmo")],
//         vec![
//             Coin::new(1, "uosmo"),
//             Coin::new(200, "usomething"),
//         ]
//     )]
//     #[case::mixed(
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(99, "uosmo"), Coin::new(200, "usomething"), Coin::new(100, "uother")],
//         vec![
//             Coin::new(1, "uosmo"),
//         ]
//     )]
//
//     pub fn test_calculate_spent_coins(
//         #[case] balances_before_spent: Vec<Coin>,
//         #[case] balances_after_spent: Vec<Coin>,
//         #[case] expected: Vec<Coin>,
//     ) {
//         let balances_before_spent = Coins::try_from(balances_before_spent).unwrap();
//         let balances_after_spent = Coins::try_from(balances_after_spent).unwrap();
//         let deltas = calculate_spent_coins(&balances_before_spent, &balances_after_spent).unwrap();
//         let expected = Coins::try_from(expected).unwrap();
//         assert_eq!(expected, deltas);
//     }
//
//     #[rstest]
//     #[case::no_delta(vec![], vec![], vec![])]
//     #[case::no_delta(vec![Coin::new(100, "uosmo")], balances_before_spent.clone(), vec![])]
//     #[case::no_delta(vec![Coin::new(100, "uosmo"), Coin::new(1023, "usomething")], balances_before_spent.clone(), vec![]
//     )]
//     #[case::receive(
//         vec![Coin::new(100, "uosmo")],
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(200, "usomething")]
//     )]
//     #[case::receive(
//         vec![Coin::new(100, "uosmo")],
//         vec![Coin::new(101, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(1, "uosmo"), Coin::new(200, "usomething")]
//     )]
//     #[case::spend(
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(99, "uosmo"), Coin::new(200, "usomething")],
//         vec![]
//     )]
//     #[case::spend(
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(99, "uosmo"), Coin::new(199, "usomething")],
//         vec![]
//     )]
//     #[case::spend(
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(99, "uosmo")],
//         vec![]
//     )]
//     #[case::mixed(
//         vec![Coin::new(100, "uosmo"), Coin::new(200, "usomething")],
//         vec![Coin::new(99, "uosmo"), Coin::new(200, "usomething"), Coin::new(100, "uother")],
//         vec![
//             Coin::new(100, "uother"),
//         ]
//     )]
//     pub fn test_calculate_received_coins(
//         #[case] balances_before_spent: Vec<Coin>,
//         #[case] balances_after_spent: Vec<Coin>,
//         #[case] expected: Vec<Coin>,
//     ) {
//         let balances_before_spent = Coins::try_from(balances_before_spent).unwrap();
//         let balances_after_spent = Coins::try_from(balances_after_spent).unwrap();
//         let deltas =
//             calculate_received_coins(&balances_before_spent, &balances_after_spent).unwrap();
//         let expected = Coins::try_from(expected).unwrap();
//         assert_eq!(expected, deltas);
//     }
// }
