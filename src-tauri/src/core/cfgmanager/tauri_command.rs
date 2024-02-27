use super::{ChangeConfigDto, ConfigDto, MakuConfigManager};


#[tauri::command(rename_all = "snake_case")]
pub fn update_config(data: ChangeConfigDto) -> String {
    let mut manager = MakuConfigManager::access();
    
    if let Some(lang) = data.lang {
        manager.config.set_lang(lang);
    }
    
    manager.save();

    "Success".to_string()
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_config() -> Result<ConfigDto, String> {
    let config = MakuConfigManager::access().config.clone();
    
    Ok(ConfigDto { lang: config.lang })
}