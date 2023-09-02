use super::service::SUBJECT_SERVICE;
use super::dto::SubjectResDto;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_subject(name: &str, description: &str, belong_category: &str) -> Result<String, String> {
    let result = SUBJECT_SERVICE
        .create_subject(name.to_string(), description.to_string(), belong_category.to_string())
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_subject(id: String, title: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, String> {
    let result = SUBJECT_SERVICE
        .update_subject(id, title, description, auth)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_all_subject() -> Result<Vec<SubjectResDto>, String> {
    let result = SUBJECT_SERVICE
        .get_all_subject()
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_subject_by_id(id: String) -> Result<Option<SubjectResDto>, String> {
    let result = SUBJECT_SERVICE
        .get_subject_by_id(id)
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn list_subjects(
    id: Option<String>,
    name: Option<String>,
    belong_category: Option<String>, 
    order_by: Option<String>,
) -> Result<Vec<SubjectResDto>, String> {
    let result = SUBJECT_SERVICE
        .list_subjects(
            id, 
            name, 
            belong_category, 
            order_by
        )
        .await?;

    Ok(result)
}