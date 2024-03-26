use crate::Result;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};
use spend_limit::msg::SwapAmountInRoute;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
struct RouterResponse {
    amount_in: Token,
    amount_out: String,
    route: Vec<Route>,
    effective_fee: String,
    price_impact: String,
    in_base_out_quote_spot_price: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub denom: String,
    pub amount: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.amount, self.denom)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Route {
    pools: Vec<Pool>,
    #[serde(rename = "has-cw-pool")]
    has_cw_pool: bool,
    out_amount: String,
    in_amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pool {
    id: u64,
    #[serde(rename = "type")]
    pool_type: u8,
    balances: Vec<String>,
    spread_factor: String,
    token_out_denom: String,
    taker_fee: String,
}

pub async fn get_route(token_in: Token, token_out_denom: &str) -> Result<Vec<SwapAmountInRoute>> {
    let url = format!(
        "https://sqsprod.osmosis.zone/router/quote?tokenIn={}&tokenOutDenom={}",
        utf8_percent_encode(token_in.to_string().as_str(), NON_ALPHANUMERIC),
        token_out_denom
    );

    let res = reqwest::get(&url).await?;
    let txt = res.text().await?;
    let response: RouterResponse = serde_json::from_str(&txt).map_err(|e| {
        format!(
            "Failed to parse response from sqs: {}. Denom: {}, Response: {}",
            e, token_in.denom, txt
        )
    })?;

    // get route with the best out amount
    let route = response
        .route
        .iter()
        .max_by(|a, b| {
            a.out_amount
                .parse::<u128>()
                .unwrap()
                .cmp(&b.out_amount.parse::<u128>().unwrap())
        })
        .expect("No route found");

    let best_route = route
        .pools
        .iter()
        .map(|pool| SwapAmountInRoute {
            pool_id: pool.id,
            token_out_denom: pool.token_out_denom.clone(),
        })
        .collect::<Vec<_>>();

    Ok(best_route)
}
