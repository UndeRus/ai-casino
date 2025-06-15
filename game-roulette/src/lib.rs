use casino_core::types::*;
use rand::{prelude::*, rng};

pub fn spin_roulette() -> RouletteSpin {
    let mut rng: ThreadRng = rng();
    let number = rng.random_range(0..=36);
    let color = match number {
        0 => Color::Green,
        n if [1,3,5,7,9,12,14,16,18,19,21,23,25,27,30,32,34,36].contains(&n) => Color::Red,
        _ => Color::Black,
    };

    RouletteSpin { number, color }
}

pub fn evaluate_bets(bets: &[Bet], spin: &RouletteSpin) -> f64 {
    bets.iter().map(|bet| {
        match &bet.bet_type {
            BetType::Straight(n) if *n == spin.number => bet.amount * 36.0,
            BetType::Color(c) if *c == spin.color => bet.amount * 2.0,
            BetType::EvenOdd(even) => {
                if spin.number != 0 && (*even == (spin.number % 2 == 0)) {
                    bet.amount * 2.0
                } else { 0.0 }
            }
            BetType::Dozen(dz) => {
                let dz_match = match dz {
                    1 => (1..=12).contains(&spin.number),
                    2 => (13..=24).contains(&spin.number),
                    3 => (25..=36).contains(&spin.number),
                    _ => false,
                };
                if dz_match { bet.amount * 3.0 } else { 0.0 }
            }
            _ => 0.0,
        }
    }).sum::<f64>() - bets.iter().map(|b| b.amount).sum::<f64>()
}

