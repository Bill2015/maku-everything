use crate::category::domain::CategoryError;

use super::service::CATEGORY_SERVICE;
use super::dto::CategoryResDto;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_category(name: &str, description: &str, root_path: &str) -> Result<String, CategoryError> {
    let result = CATEGORY_SERVICE
        .create_category(name.to_string(), description.to_string(), root_path.to_string())
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn update_category(id: String, name: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, CategoryError> {
    let result = CATEGORY_SERVICE
        .update_category(id, name, description, auth)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_all_category() -> Result<Vec<CategoryResDto>, CategoryError> {
    let result = CATEGORY_SERVICE
        .get_all_category()
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_category_by_id(id: String) -> Result<Option<CategoryResDto>, CategoryError> {
    let result = CATEGORY_SERVICE
        .get_category_by_id(id)
        .await?;

    Ok(result)
}
