use crate::modules::tag::domain::{TagError, TagID};

use super::service::TAG_SERVICE;
use super::dto::*;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_tag(data: CreateTagDto) -> Result<TagID, TagError> {
    let result = TAG_SERVICE
        .create_tag(data)
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_tag(data: UpdateTagDto) -> Result<TagID, TagError> {
    let result = TAG_SERVICE
        .update_tag(data)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_all_tag() -> Result<Vec<TagResDto>, TagError> {
    let result = TAG_SERVICE
        .get_all_tag()
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_tag_by_id(id: String) -> Result<Option<TagResDto>, TagError> {
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
    belong_subject_name: Option<String>,
    tagging_resource: Option<String>,
    order_by: Option<String>,
) -> Result<Vec<TagResDto>, TagError> {
    let result = TAG_SERVICE
        .list_tags(
            id, 
            name, 
            belong_category, 
            belong_subject, 
            belong_subject_name,
            tagging_resource, 
            order_by
        )
        .await?;

    Ok(result)
}