use crate::resource::domain::ResourceError;

use super::{service::RESOURCE_SERVICE, dto::{ResourceResDto, ResourceDetailDto}};

#[tauri::command(rename_all = "snake_case")]
pub async fn create_resource(name: &str, description: &str, file_path: &str, url_path: &str, belong_category: &str) -> Result<String, ResourceError> {
    let result = RESOURCE_SERVICE
        .create_resource(
            name.to_string(),
            description.to_string(),
            file_path.to_string(),
            url_path.to_string(),
            belong_category.to_string()
        )
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_resource(id: String, name: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, ResourceError> {
    let result = RESOURCE_SERVICE
        .update_resource(id, name, description, auth)
        .await?;

    Ok(result)
}


#[tauri::command(rename_all = "snake_case")]
pub async fn add_resource_tag(id: String, tag_id: String) -> Result<String, ResourceError> {
    let result = RESOURCE_SERVICE
        .add_resource_tag(id, tag_id)
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn remove_resource_tag(id: String, tag_id: String) -> Result<String, ResourceError> {
    let result = RESOURCE_SERVICE
        .remove_resource_tag(id, tag_id)
        .await?;

    Ok(result)
}


#[tauri::command]
pub async fn get_all_resource() -> Result<Vec<ResourceResDto>, ResourceError> {
    let result = RESOURCE_SERVICE
        .get_all_resource()
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_resource_by_id(id: String) -> Result<Option<ResourceResDto>, ResourceError> {
    let result = RESOURCE_SERVICE
        .get_resource_by_id(id)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_resource_detail(id: String) -> Result<Option<ResourceDetailDto>, ResourceError> {
    let result = RESOURCE_SERVICE
        .resource_detail(id)
        .await?;

    Ok(result)
}

// Related Problem https://github.com/tauri-apps/tauri/issues/4062#issuecomment-1118394619
#[tauri::command(rename_all = "snake_case")]
pub async fn explore_the_file(file_path: String) -> Result<(), ResourceError> {
    let _ = RESOURCE_SERVICE
        .expore_the_file(file_path)
        .await?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_resource(
    id: Option<String>,
    name: Option<String>,
    belong_category: Option<String>, 
    order_by: Option<String>,
) -> Result<Vec<ResourceResDto>, ResourceError> {
    let result = RESOURCE_SERVICE
        .list_resource(
            id, 
            name, 
            belong_category, 
            order_by
        )
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn querying_by_string(q: String) -> Result<Vec<ResourceResDto>, ResourceError> {
    let result = RESOURCE_SERVICE
        .querying_by_string(q)
        .await?;

    Ok(result)
}
