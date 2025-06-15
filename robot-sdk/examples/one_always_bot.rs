use casino_core::types::{Bet, BetType, Color, RouletteSpin};

use crate::Robot;

pub struct OneAlwaysBot {
    id: usize,
}

impl OneAlwaysBot {
    pub fn new(player_id: usize) -> Self {
        Self { id: player_id }
    }
}

impl Robot for OneAlwaysBot {
    fn name(&self) -> &str {
        "OneAlwaysBot"
    }

    fn place_bets(
        &mut self,
        _history: &[RouletteSpin],
        balance: f64,
    ) -> Result<Vec<Bet>, &'static str> {
        Ok(vec![Bet {
            bet_type: BetType::Straight(1),
            amount: (balance * 0.05).min(50.0),
        }])
    }
}
