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
#[derive(Debug)]

pub struct ReplaceColors<T> {
    pub primary: T,
    pub secondary: T,
    pub tertiary: T,
}

impl<T> ReplaceColors<T> {
    pub fn get_params() -> Vec<String> {
        vec!["primary", "secondary", "tertiary"].iter().map(|x| x.to_string()).collect()
    }
    pub fn get_pairs(&self) -> Vec<(String, &T)> {
        vec![
            ("primary".into()  ,  &self.primary),
            ("secondary".into(), &self.secondary),
            ("tertiary".into() , &self.tertiary)
        ]
    }
}

impl ReplaceFile {
    pub fn replace(&self, colors: &ReplaceColors<String>) {
        let mut file_in = fs::File::open(self.from.clone())
            .expect(&format!("Failed to open 'from' file {:?}", self.from));
        let mut file_out = fs::File::options()
            .write(true)
            .open(self.to.clone())
            .expect(&format!("Failed to open 'to' file {:?}", self.from));
        let mut data = String::new();
        file_in
            .read_to_string(&mut data)
            .expect(&format!("Failed to read 'from' file {:?}", self.from));

        for (key, color) in colors.get_pairs() {
            data = data.replace(&Config::replace_key(key), color);
        }
        file_out
            .write_all(data.as_bytes())
            .expect(&format!("Failed to write into file {:?}", self.to));
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
                    files: vec![
                        ReplaceFile {
                            from: "/example/path/from".into(),
                            to: "/example/path/to".into(),
                        },
                        ReplaceFile {
                            from: "/example/path/from2".into(),
                            to: "/example/path/to2".into(),
                        },
                    ],
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
        Config::from_path(config_file)
    }

    pub fn from_path(path: PathBuf) -> Config {
        let mut file = fs::File::open(&path).expect(&format!("Couldn't open file {:?}", path));
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect(&format!("Couldn't read file {:?}", path));
        serde_json::from_str(&buf)
            .expect(&format!("Failed to serialize config file {:?}", path))
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

    pub fn process(&self, colors: &ReplaceColors<String>) {
        for file in &self.files {
            file.replace(colors);
        }
    }

    pub fn get_files(&self) -> Vec<ReplaceFile> {
        self.files.clone()
    }

    pub fn replace_key(key: String) -> String {
        format!("$[{key}]")
    }
}
