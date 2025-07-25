use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

const APP_KEY: &'static str = "colors_replacer";

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    warning: FirstTimeStruct,
    files: Vec<ReplaceFile>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ReplaceFile {
    pub from: PathBuf,
    pub to: PathBuf,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FirstTimeStruct {
    first_time: bool,
    text: String,
}

impl ReplaceFile {
    pub fn replace_files(&self, colors: &Vec<String>) {
        let mut file_in = fs::File::open(self.from.clone()).expect(&format!("Failed to open 'from' file {:?}", self.from));
        let mut file_out = fs::File::options().write(true).open(self.to.clone()).expect(&format!("Failed to open 'to' file {:?}", self.from));
        let mut data = String::new();
        file_in.read_to_string(&mut data).expect(&format!("Failed to read 'from' file {:?}", self.from));
        for i in 0..colors.len() {
           data = data.replace(&Config::replace_key(i as u8), &colors[i as usize]);
        }
        file_out.write_all(data.as_bytes()).expect(&format!("Failed to write into file {:?}", self.to));
    }
}

impl Config {
    pub fn new() -> Config {
        let mut path = dirs::config_dir().expect("Couldn't get path for config directory");
        path.push(APP_KEY);
        if !path.exists() {
            fs::create_dir(&path).unwrap();
        };
        let config_file = Config::get_config_path();
        if !config_file.exists() {
            let mut file = fs::File::create(&config_file).unwrap();
            file.write_all(
                serde_json::to_string_pretty(&Config {
                    files: vec![ReplaceFile {
                        from: "/example/path/from".into(),
                        to: "/example/path/to".into(),
                    }, ReplaceFile {
                        from: "/example/path/from2".into(),
                        to: "/example/path/to2".into(),
                    }],
                    warning: FirstTimeStruct {
                        first_time: true,
                        text: "Set first_time to false in order to continue!".to_string(),
                    },
                })
                .unwrap()
                .as_bytes(),
            )
            .unwrap();
            return Config {
                files: vec![],
                warning: FirstTimeStruct {
                    first_time: true,
                    text: "Set first_time to false in order to continue!".to_string(),
                },
            };
        };
        let mut file =
            fs::File::open(&config_file).expect(&format!("Couldn't open file {:?}", config_file));
        let cfg: Config = {
            let mut buf = String::new();
            serde_json::from_str({
                file.read_to_string(&mut buf)
                    .expect(&format!("Couldn't read file {:?}", config_file));
                &buf.clone()
            })
            .expect(&format!(
                "Failed to serialize config file {:?}",
                config_file
            ))
        };
        cfg
    }
    pub fn is_first_time(&self) -> bool {
        self.warning.first_time
    }
    pub fn get_config_path() -> PathBuf {
        let path = dirs::config_dir().expect("Couldn't get path for config directory");
        let mut path = path.clone();
        path.push(APP_KEY);
        path.push("config.json");
        path
    }

    pub fn process(&self, colors: &Vec<String>) {
        for file in &self.files {
            file.replace_files(colors);
        }
    }

    pub fn get_files(&self) -> Vec<ReplaceFile> {
        self.files.clone()
    }
    
    pub fn replace_key(key: u8) -> String {
        format!("$[{key}]")
    }
}
