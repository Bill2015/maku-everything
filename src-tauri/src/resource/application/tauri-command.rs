use super::{service::RESOURCE_SERVICE, dto::ResourceResDto};

#[tauri::command]
pub async fn create_resource(title: &str, description: &str, file_path: &str, belong_resource: &str) -> Result<String, String> {
    let result = RESOURCE_SERVICE
        .create_resource(title.to_string(), description.to_string(), file_path.to_string(), belong_resource.to_string())
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

#[tauri::command]
pub async fn get_all_resource() -> Result<Vec<ResourceResDto>, String> {
    let result = RESOURCE_SERVICE
        .get_all_resource()
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_resource_by_id(id: String) -> Result<Option<ResourceResDto>, String> {
    let result = RESOURCE_SERVICE
        .get_resource_by_id(id)
        .await?;

    Ok(result)
}
