use crate::category::domain::CategoryError;

use super::service::CATEGORY_SERVICE;
use super::dto::{CategoryResDto, CreateCategoryDto, UpdateCategoryDto};

#[tauri::command(rename_all = "snake_case")]
pub async fn create_category(data: CreateCategoryDto) -> Result<String, CategoryError> {
    let result = CATEGORY_SERVICE
        .create(data)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn update_category(data: UpdateCategoryDto) -> Result<String, CategoryError> {
    let result = CATEGORY_SERVICE
        .update(data)
        .await?;

    Ok(result)
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
