use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex, MutexGuard};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

mod dto;
pub use dto::*;
mod tauri_command;
pub use tauri_command::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MakuConfig {
    lang: String,
}

impl Default for MakuConfig {
    fn default() -> Self {
        Self { lang: "enUS".to_string() }
    }
}

impl MakuConfig {
    pub fn set_lang(&mut self, lang: String) {
        if lang == "enUS" || lang == "zhTW" {
            self.lang = lang;
        }
    }
}

static MAKU_CONFIG: OnceCell<Mutex<MakuConfigManager>> = OnceCell::new();

#[derive(Debug)]
pub struct MakuConfigManager {
    config: MakuConfig,

    path: String,
}

impl MakuConfigManager {

    pub fn init(config_path: String) -> Result<(), String> {
        fs::create_dir_all(&config_path)
            .expect("config directory should be writable");

        let path = Path::new(&config_path).join("config.json");

        let mut manager = Self {
            config: MakuConfig::default(),
            path: path.to_str().unwrap().to_string(),
        };
        manager.load();

        MAKU_CONFIG.set(Mutex::new(manager))
            .expect("config manager is initialize failed");
        Ok(())
    }

    
    pub fn save(&self) {
        let data = serde_json::to_string(&self.config)
            .unwrap();

        let mut writer = File::create(&self.path)
            .expect("cannot create the file writer");

        writer
            .write_all(data.as_bytes())
            .expect("config should be writable");
    }

    pub fn load(&mut self) {
        let path = Path::new(&self.path);

        if path.exists() == false {
            self.config = MakuConfig::default();
            self.save();
            return;
        }
        
        let reader = File::open(path);
        let mut buffer = String::new();

        match reader {
            Ok(mut file) => {
                file.read_to_string(&mut buffer)
                    .expect("config should be readable");
            },
            Err(e) => {
                buffer = serde_json::to_string(&MakuConfig::default()).unwrap();
            }
        }

        self.config = serde_json::from_str(&buffer).unwrap();
    }

    pub fn access() -> MutexGuard<'static, MakuConfigManager> {
        MAKU_CONFIG.get().unwrap().lock().unwrap()
    }
}