use std::fs;
use crate::adb;
use serde_json::{*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    script_name: String,
}

impl Language {
    pub fn new() -> Self {
        let language_data = fs::read_to_string(format!("{}language.json", adb::Exist::get_path())).expect("Can't read language data file.");
        let language_data_parsed: Language = from_str(&language_data).expect("Can't parse language data. Language data might be broken.");
        Language {
            script_name: language_data_parsed.script_name,
        }
    }
}