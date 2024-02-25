use std::path::Path;

use serde::Serialize;

use crate::modules::resource::domain::ResourceGenericError;
use crate::utils::StringUtils;

#[derive(Debug, Serialize, Clone)]
pub struct ResourceFileVO {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub ext: Option<String>,
}

impl ResourceFileVO {
    pub fn new(root_path: &String, file_path: String) -> Result<Self, ResourceGenericError> {
        // If path already contain root path
        // trim it and re-concat it
        let main_path = file_path.trim_start_matches(root_path);
        let full_path = format!("{}{}", root_path, main_path);

        // concat with root path
        let path = Path::new(full_path.as_str());

        if path.exists() == false {
            return Err(ResourceGenericError::FilePathNotExist());
        }
        
        if path.file_name().is_none() {
            return Err(ResourceGenericError::FileNameIsEmpty());
        }

        let ext = match path.is_file() {
            true => path.extension()
                .map(|osr| Some(String::from(osr.to_str().unwrap())))
                .unwrap_or(None),
            false => Some("folder".to_string()),
        };

        let name = String::from(path.file_name().unwrap().to_str().unwrap());
        let name = match path.is_file() {
            true if ext.is_none()=> name,
            true => String::from(name.slice(..name.chars().count() - ext.as_ref().unwrap().chars().count() - 1)),
            false => name,
        };

        Ok(
            ResourceFileVO {
                uuid: String::from("id"),
                name: name,
                ext: ext,
                path: String::from(main_path),
            }
        )
    }
}
