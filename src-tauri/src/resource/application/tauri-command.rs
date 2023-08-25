use super::service::RESOURCE_SERVICE;

#[tauri::command]
pub async fn create_resource(title: &str, description: &str, file_path: &str, belong_category: &str) -> Result<String, String> {
    let result = RESOURCE_SERVICE
        .create_resource(title.to_string(), description.to_string(), file_path.to_string(), belong_category.to_string())
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn update_resource(id: String, title: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, String> {
    let result = RESOURCE_SERVICE
        .update_resource(id, title, description, auth)
        .await?;

    Ok(result)
}
