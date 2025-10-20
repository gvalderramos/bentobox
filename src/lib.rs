use std::path::PathBuf;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "user-dev-folder")]
    pub user_dev_folder: String,
    #[serde(rename = "ide-cmd")]
    pub ide_command: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BentoBoxConfig { pub bentobox: Config }

pub fn get_default_config_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(dirs::home_dir().unwrap());
    path.push(".bentobox");

    if !path.exists() {
        fs::create_dir_all(&path).expect("failed to create config directory");
    }
    path.push("bentobox.yaml");

    if !path.exists() {
        let file = fs::File::create(&path).expect("Unable to create the config file.");
        let basic_config = Config{user_dev_folder: "~/Documents/dev".to_string(), ide_command: "code".to_string()};
        let config = BentoBoxConfig{bentobox: basic_config};
        serde_yaml::to_writer(file, &config).expect("Error: Unable to write ");
    }
    path
}

pub fn get_config_content() -> BentoBoxConfig {
    let path = get_default_config_path();

    let file = fs::File::open(path).expect("Unable to read the config content path");
    let content : BentoBoxConfig = serde_yaml::from_reader(file).expect("Unable to read the config file content");
    // let map_values : BentoBoxConfig = serde_yaml::from_str(&content).unwrap();

    content
}