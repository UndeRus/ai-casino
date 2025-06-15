use casino_core::types::*;
use game_roulette::*;
use robot_sdk::Robot;

pub struct SimulationResult {
    pub final_balances: Vec<f64>,
    pub history: Vec<RouletteSpin>,
    pub total_spins: usize,
}

pub struct SingleSimulationResult {
    pub final_balance: f64,
    pub history: Vec<RouletteSpin>,
    pub total_spins: usize,
}

pub fn simulate_game_table(robots: &mut [Box<dyn Robot>], spins: usize) -> SimulationResult {
    let mut balance = 1000.0;
    let mut history = vec![];

    let mut spin_number = 0;

    let mut final_balances = vec![balance; robots.len()];

    for _ in 0..spins {
        let spin = spin_roulette();
        for (index, robot) in robots.iter_mut().enumerate() {
            let bets = robot.place_bets(&history, balance).unwrap();


            let spin = spin.clone();

            let payout = evaluate_bets(&bets, &spin).floor();
            
            
            final_balances[index] += payout;
            history.push(spin);
            if balance <= 0.0 {
                continue;
            }
        }
        spin_number += 1;
    }

    SimulationResult {
        history: history,
        final_balances,
        total_spins: spin_number,
    }
}

pub fn simulate_game(robot: &mut Box<dyn Robot>, spins: usize) -> SingleSimulationResult {
    let mut balance = 1000.0;
    let mut history = vec![];

    let mut spin_number = 0;

    for _ in 0..spins {
        let bets = robot.place_bets(&history, balance).unwrap();
        let spin = spin_roulette();
        let payout = evaluate_bets(&bets, &spin).floor();
        balance += payout;
        history.push(spin);

        spin_number += 1;

        if balance <= 0.0 {
            break;
        }
    }

    SingleSimulationResult {
        history: history,
        final_balance: balance,
        total_spins: spin_number,
    }
}
