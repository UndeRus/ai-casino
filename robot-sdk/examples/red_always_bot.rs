use casino_core::types::{Bet, BetType, Color, RouletteSpin};

use crate::Robot;

pub struct RedAlwaysBot {
    id: usize,
}

impl RedAlwaysBot {
    pub fn new(player_id: usize) -> Self {
        Self { id: player_id }
    }
}

impl Robot for RedAlwaysBot {
    fn name(&self) -> &str {
        "RedAlwaysBot"
    }

    fn place_bets(
        &mut self,
        _history: &[RouletteSpin],
        balance: f64,
    ) -> Result<Vec<Bet>, &'static str> {
        Ok(vec![Bet {
            bet_type: BetType::Color(Color::Red),
            amount: (balance * 0.05).min(50.0),
        }])
    }
}
