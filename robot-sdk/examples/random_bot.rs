use casino_core::types::{Bet, BetType, Color, RouletteSpin};
use rand::{rng, seq::IndexedRandom, thread_rng};

use crate::Robot;

pub struct RandomBot {
    id: usize,
    variants: Vec<BetType>,
}

impl RandomBot {
    pub fn new(player_id: usize) -> Self {
        Self {
            id: player_id,
            variants: vec![BetType::Color(Color::Red), BetType::Color(Color::Black)],
        }
    }
}

impl Robot for RandomBot {
    fn name(&self) -> &str {
        "RedAlwaysBot"
    }

    fn place_bets(
        &mut self,
        _history: &[RouletteSpin],
        balance: f64,
    ) -> Result<Vec<Bet>, &'static str> {
        let bet = self.variants.choose(&mut rng()).ok_or("Failed to place bets")?.clone();
        Ok(vec![Bet {
            bet_type: bet, //BetType::Color(Color::Red),
            amount: (balance * 0.05).min(50.0),
        }])
    }
}
