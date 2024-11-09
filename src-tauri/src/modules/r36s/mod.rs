use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmulatorData {
    pub name: &'static str,
    pub formats: Vec<&'static str>,
}

pub fn get_emulator_data() -> Vec<EmulatorData> {
    vec![
        EmulatorData {
            name: "nes",
            formats: vec![".nes", ".zip"],
        },
        EmulatorData {
            name: "snes",
            formats: vec![".sfc", ".smc", ".zip"],
        },
        EmulatorData {
            name: "gbc",
            formats: vec![".gbc", ".zip"],
        },
        EmulatorData {
            name: "gba",
            formats: vec![".gba", ".zip"],
        },
        EmulatorData {
            name: "n64",
            formats: vec![".n64", ".v64", ".z64", ".zip"],
        },
        EmulatorData {
            name: "nds",
            formats: vec![".nds", ".zip"],
        },
    ]
}


