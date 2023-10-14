use std::collections::HashMap;

use serde_json::Value;


pub struct Config {
    default: Driver,
    drivers: HashMap<String, Value>
}

impl Config {
    pub fn load_config(init: &str) -> Self {
        Config { config: serde_json::from_str(init).expect("drivers config is not in currect format") }
    }
    pub fn get(&self, drv: String) -> Option<&Value> {
        self.config.get(&drv)
    }
}
