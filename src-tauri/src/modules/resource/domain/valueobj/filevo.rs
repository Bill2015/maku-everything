use std::ffi::OsStr;
use std::path::Path;

use serde::Serialize;

use crate::modules::resource::domain::ResourceGenericError;

#[derive(Debug, Serialize)]
pub struct ResourceFileVO {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub ext: String,
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
            true => path.extension().unwrap_or(OsStr::new("txt")),
            false => OsStr::new("folder"),
        };

        Ok(
            ResourceFileVO {
                uuid: String::from("id"),
                name: String::from(path.file_name().unwrap().to_str().unwrap()),
                ext: String::from(ext.to_str().unwrap()),
                path: String::from(main_path),
            }
        )
    }
}
