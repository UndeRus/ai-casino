/*
use extism::{Plugin, Manifest};
use crate::wasm_bot::{BotInput, BotOutput, BotBet, BotRunner};
use anyhow::Result;

pub struct WasmBot {
    plugin: Plugin,
    name: String,
}

impl WasmBot {
    pub fn load(path: &str) -> Result<Self> {
        let manifest = Manifest::new([path]);
        let mut plugin = Plugin::new(manifest, [], true)?;

        let name = plugin.call::<_, String>("name", ())?;
        Ok(Self { plugin, name })
    }
}

impl BotRunner for WasmBot {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn place_bets(&mut self, history: &[u8], balance: f64) -> Vec<BotBet> {
        let input = BotInput {
            history: history.to_vec(),
            balance,
        };
        let input_json = serde_json::to_string(&input).unwrap();

        let out: BotOutput = self.plugin.call("place_bets", input_json).unwrap();
        out.bets
    }
}
*/