use engine::{simulate_game, simulate_game_table};
use robot_sdk::Robot;
use robot_wasm_loader::WasmLoader;

fn main() {
    println!("Start balance 1000");
    run_table();
}

fn run_single() {
    let mut bot2: Box<dyn Robot> =
        Box::new(WasmLoader::load("target/wasm32-unknown-unknown/release/redalways.wasm").unwrap());
    let result = simulate_game(&mut bot2, 1000);
    println!(
        "Simulation complete!\nFinal balance: {:.2} after {} spins.",
        result.final_balance, result.total_spins
    );
}

fn run_table() {
    let bot2: Box<dyn Robot> =
        Box::new(WasmLoader::load("target/wasm32-unknown-unknown/release/redalways.wasm").unwrap());
    let bot1 = Box::new(
        WasmLoader::load("target/wasm32-unknown-unknown/release/blackalways.wasm").unwrap(),
    );

    let result = simulate_game_table(&mut [bot1, bot2], 1000);
    println!(
        "Simulation complete!\nFinal balance: {:?} after {} spins.",
        result.final_balances, result.total_spins
    );
}
