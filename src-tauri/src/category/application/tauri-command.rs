use crate::category::domain::{CategoryError, CategoryID};

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

#[tauri::command(rename_all = "snake_case")]
pub async fn import_category(data: ImportCategoryDto) -> Result<CategoryID, CategoryError> {
    let result = CATEGORY_SERVICE
        .import(data)
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn export_category() {

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
