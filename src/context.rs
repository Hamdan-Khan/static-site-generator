use std::fs;
use serde::{Deserialize, Serialize};
use tera::Context;
use crate::CONFIG_FILE;

#[derive(Deserialize, Serialize)]
struct Config {
    name: String,
    bio: String,
    email: String,
    description: String,
    baseurl: Option<String>,
}

pub fn get_config() -> Result<Context, tera::Error>
{
    let config = fs::read_to_string(CONFIG_FILE)
        .expect("Error reading configuration (config.yaml) from root dir");

    let yaml : Config = serde_yaml::from_str(config.as_str())
        .expect("Error parsing YAML");
    
    return Context::from_serialize(yaml);
}