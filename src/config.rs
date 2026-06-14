use serde::Deserialize; //it is to convert the json to rust struct objects
use std::collections::HashMap; //it is to use hashmap
use std::fs; //reacing the file

#[derive(Deserialize)]
pub struct Config{
    pub app: HashMap<String ,String>,
}

pub fn load_config() -> Config{
    let data = fs::read_to_string("config.json")
        .expect("Failed to read the config.json");

    let apps: HashMap<String,String> =
        serde_json::from_str(&data)
        .expect("Invalid json");

    Config{apps};

}