use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub chromedriver_path: String,
    pub url: String,
    pub welcome_div_id: String,
    pub form_div_id: String,
    pub form_input_id: String,
    pub form_button_id: String,
    pub form_score_guess_xpath: String,
    pub cool_score_xpath: String,
    pub hot_score_xpath: String,
    pub words_path: String,
    pub api_similar_endpoint: String,
    pub randomize_starting_list: bool,
}

pub fn build_config() -> Config {
    let file = fs::File::open("config.json").expect("JSON config file unreachable");
    let config: Config = serde_json::from_reader(file).expect("JSON was not well formatted");
    return config;
}
