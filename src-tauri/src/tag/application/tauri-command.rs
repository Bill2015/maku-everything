use super::service::TAG_SERVICE;
use super::dto::TagResDto;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_tag(name: &str, description: &str, belong_category: &str, belong_subject: &str) -> Result<String, String> {
    let result = TAG_SERVICE
        .create_tag(
            name.to_string(), 
            description.to_string(), 
            belong_category.to_string(),
            belong_subject.to_string(),
        )
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_tag(id: String, name: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, String> {
    let result = TAG_SERVICE
        .update_tag(id, name, description, auth)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_all_tag() -> Result<Vec<TagResDto>, String> {
    let result = TAG_SERVICE
        .get_all_tag()
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_tag_by_id(id: String) -> Result<Option<TagResDto>, String> {
    let result = TAG_SERVICE
        .get_tag_by_id(id)
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_tags(
    id: Option<String>,
    name: Option<String>,
    belong_category: Option<String>, 
    belong_subject: Option<String>,
    tagging_resource: Option<String>,
    order_by: Option<String>,
) -> Result<Vec<TagResDto>, String> {
    let result = TAG_SERVICE
        .list_tags(
            id, 
            name, 
            belong_category, 
            belong_subject, 
            tagging_resource, 
            order_by
        )
        .await?;

    Ok(result)
}