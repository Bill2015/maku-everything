use std::path::Path;

use file_format::FileFormat;
use serde::Serialize;

use crate::modules::resource::domain::ResourceGenericError;
use crate::utils::StringUtils;

#[derive(Debug, Serialize, Clone)]
pub struct ResourceFileVO {
    pub uuid: String,
    pub name: String,
    pub path: String,
    pub ext: Option<String>,
    pub media_type: String,
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

        let obj = match path.is_file() {
            true => Self::handle_file(main_path, path),
            false => Self::handle_folder(main_path, path),
        }?;

        Ok(obj)
    }


    fn handle_folder(main_path: &str, path: &Path) -> Result<Self, ResourceGenericError> {
        let name = String::from(path.file_name().unwrap().to_str().unwrap());
        
        Ok(ResourceFileVO {
            uuid: String::from("id"),
            name: name,
            ext: None,
            path: main_path.to_string(),
            media_type: "application/folder".to_string(),
        })
    }

    fn handle_file(main_path: &str, path: &Path) -> Result<Self, ResourceGenericError> {
        let filefmt = FileFormat::from_file(path)
            .or(Err(ResourceGenericError::FilePathNotExist()))?;

        let media_type = filefmt.media_type().to_string();
        let ext = path.extension()
            .map(|osr| Some(String::from(osr.to_str().unwrap())))
            .unwrap_or(None);

        let name = String::from(path.file_name().unwrap().to_str().unwrap());
        let name = match ext.is_none() {
            true => name,
            false => String::from(name.slice(..name.chars().count() - ext.as_ref().unwrap().chars().count() - 1)),
        };

        Ok(ResourceFileVO {
            uuid: String::from("id"),
            name: name,
            ext: ext,
            path: main_path.to_string(),
            media_type: media_type,
        })
    }
}
