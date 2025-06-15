use casino_core::types::{Bet, BetType, Color};
use extism_pdk::{plugin_fn, FnResult, Json};
use robot_sdk::{BotInput, BotOutput};


#[plugin_fn]
pub fn name(_: ()) -> FnResult<String> {
    Ok("RedAlwaysBot".into())
}

#[plugin_fn]
pub fn place_bets(input: Json<BotInput>) -> FnResult<Json<BotOutput>> {
    Ok(Json(BotOutput {
        bets: vec![Bet {
            bet_type: BetType::Color(Color::Red),
            amount: 10.0,
        }],
    }))
}
