use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
extern crate serde_yaml;

const CONFIG_PATHS: &[&str] = &["/etc/rws/config.yml", ".config/rws/config.yml"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    base: String,
    port: u16,
    bind: Vec<String>,
}

fn get_home_dir() -> String {
    match env::var("HOME") {
        Ok(home) => home,
        Err(_) => "".to_owned(),
    }
}

impl Config {
    pub fn get_base(&self) -> String {
        self.base.clone()
    }

    pub fn get_bind_address(&self) -> String {
        format!(
            "{}:{}",
            self.bind.first().unwrap_or(&"127.0.0.1".to_owned()),
            self.port
        )
    }

    pub fn read() -> Result<Config, String> {
        let home_dir = get_home_dir();
        let home_defined = home_dir.len() > 0;

        for file in CONFIG_PATHS {
            let config_file = if home_defined && !file.starts_with("/") {
                let tmp = format!("{}/{}", home_dir, file);
                tmp.to_owned()
            } else {
                file.to_string()
            };

            match File::open(config_file.clone()) {
                Ok(yaml) => {
                    let reader = BufReader::new(yaml);
                    match serde_yaml::from_reader::<_, Config>(reader) {
                        Ok(config) => {
                            println!("Found {}", config_file);
                            return Ok(config);
                        }
                        Err(msg) => return Err(format!("Error reading {}: {}", config_file, msg)),
                    };
                }
                Err(msg) => println!("Could not open {}: {}", config_file, msg),
            };
        }

        Err("No config file found".to_owned())
    }
}
