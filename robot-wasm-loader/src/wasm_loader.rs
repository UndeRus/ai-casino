use std::time::Duration;

use casino_core::types::{Bet, RouletteSpin};
use extism::{Manifest, Plugin, convert::Json};
use robot_sdk::{BotInput, BotOutput, Robot};

pub struct WasmLoader {
    plugin: Plugin,
    name: String,
}

impl WasmLoader {
    pub fn load(path: &str) -> Result<Self, &'static str> {
        let wasm_bytes = std::fs::read(path).map_err(|e| {
            eprintln!("Error {e}");
            "Failed to load bytes for wasm bot"
        })?;
        let manifest = Manifest::new([wasm_bytes]).with_timeout(Duration::from_secs(5));
        let mut plugin = Plugin::new(manifest, [], true).map_err(|e| {
            eprintln!("Error {e}");
            "Failed to create plugin for wasm bot"
        })?;

        let name = plugin
            .call::<_, String>("name", ())
            .map_err(|_| "Failed to get bot name")?;
        Ok(Self { plugin, name })
    }
}

impl Robot for WasmLoader {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn place_bets(
        &mut self,
        history: &[RouletteSpin],
        balance: f64,
    ) -> Result<Vec<Bet>, &'static str> {
        let input = BotInput {
            history: history.to_vec(),
            balance,
        };
        let input_json =
            serde_json::to_string(&input).map_err(|_| "Failed to serialize bot input")?;
        let out: Json<BotOutput> = self.plugin.call("place_bets", input_json).map_err(|e| {
            eprintln!("Error {e}");
            "Failed to call place bets"
        })?;
        Ok(out.0.bets)
    }
}
