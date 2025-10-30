use std::fs;
use toml;

fn get_config_contents(filepath: &str) -> String {
    if !std::path::Path::exists(std::path::Path::new(&filepath)) {
        println!("({}): Config file not found at `{}`.", colored::Colorize::red("Error"), filepath);
        std::process::exit(1);
    }

    fs::read_to_string(filepath).unwrap()
}

fn parse_config(data: &String) -> Config {
    match toml::from_str(&data) {
        Ok(out) => out,
        Err(_) => {
            println!("({}): Incorrectly configured config file.", colored::Colorize::red("Error"));
            std::process::exit(1);
        }
    }
}

pub fn get_default_config() -> Config {
    parse_config(&get_config_contents(&shellexpand::tilde("~/.config/splitzer/config.toml").to_string()))
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub key: String,
    pub model: String
}
