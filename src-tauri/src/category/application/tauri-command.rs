use super::service::CATEGORY_SERVICE;
use super::dto::{CategoryError, CategoryResDto};

#[tauri::command]
pub async fn create_category(title: &str, description: &str) -> Result<String, CategoryError> {
    let result = CATEGORY_SERVICE
        .create_category(title.to_string(), description.to_string())
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn update_category(id: String, title: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, CategoryError> {
    let result = CATEGORY_SERVICE
        .update_category(id, title, description, auth)
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
