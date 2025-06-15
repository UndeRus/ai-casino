use casino_core::types::*;
use serde::{Deserialize, Serialize};


mod wasm_bot;
mod wasm_extism;

pub trait Robot {
    fn name(&self) -> &str;
    fn place_bets(&mut self, history: &[RouletteSpin], balance: f64) -> Result<Vec<Bet>, &'static str>;
}

#[derive(Serialize, Deserialize)]
pub struct BotInput {
    pub history: Vec<RouletteSpin>,
    pub balance: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotBet {
    pub kind: String,
    pub value: u8,
    pub amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct BotOutput {
    pub bets: Vec<Bet>,
}