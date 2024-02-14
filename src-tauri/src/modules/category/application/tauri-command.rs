use std::fs::File;
use std::io::Write;

use anyhow::anyhow;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use tauri::api::dialog::blocking::FileDialogBuilder;

use crate::modules::category::domain::{CategoryError, CategoryID};

use super::service::CATEGORY_SERVICE;
use super::dto::*;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_category(data: CreateCategoryDto) -> Result<CategoryID, CategoryError> {
    let result = CATEGORY_SERVICE
        .create(data)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn update_category(data: UpdateCategoryDto) -> Result<CategoryID, CategoryError> {
    let result = CATEGORY_SERVICE
        .update(data)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn update_mapper_rule_category(data: UpdateCategoryMapperRuleDto) -> Result<CategoryID, CategoryError> {
    let result = CATEGORY_SERVICE
        .update_rules(data)
        .await?;

    Ok(result)
}


#[tauri::command(rename_all = "snake_case")]
pub async fn import_category(data: ImportCategoryDto) -> Result<CategoryID, CategoryError> {
    let result = CATEGORY_SERVICE
        .import(data)
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_category(data: ExportCategoryDto) -> Result<String, CategoryError> {
    let id = data.id.clone();
    let result = CATEGORY_SERVICE
        .export(data)
        .await?;

    let save = FileDialogBuilder::new()
        .set_title("Save file")
        .add_filter("maku", &["maku"])
        .save_file()
        .ok_or(CategoryError::Export(anyhow!("Save file failed")))?;

    let mut file = File::create(save).expect("Unable to create file");
    file.write_all(BASE64_STANDARD.encode(result.as_bytes()).as_bytes()).expect("Unable to write to file");

    Ok(id)
}

#[tauri::command]
pub async fn get_all_category() -> Result<Vec<CategoryResDto>, CategoryError> {
    let result = CATEGORY_SERVICE
        .get_all()
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_category_by_id(id: String) -> Result<Option<CategoryResDto>, CategoryError> {
    let result = CATEGORY_SERVICE
        .get_by_id(id)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_category_mapper_rules(id: String) -> Result<Option<CategoryMapperRulesResDto>, CategoryError> {
    let result = CATEGORY_SERVICE
        .get_mapper_rules(id)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn list_categories(data: QueryCategoryDto) -> Result<Vec<CategoryResDto>, CategoryError> {
    let result = CATEGORY_SERVICE
        .list_categories(data)
        .await?;

    Ok(result)
}
