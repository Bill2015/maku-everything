use crate::subject::domain::{SubjectError, SubjectID};

use super::service::SUBJECT_SERVICE;
use super::dto::*;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_subject(data: CreateSubjectDto) -> Result<SubjectID, SubjectError> {
    let result = SUBJECT_SERVICE
        .create_subject(data)
        .await?;

    Ok(result)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_subject(data: UpdateSubjectDto) -> Result<SubjectID, SubjectError> {
    let result = SUBJECT_SERVICE
        .update_subject(data)
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_all_subject() -> Result<Vec<SubjectResDto>, SubjectError> {
    let result = SUBJECT_SERVICE
        .get_all_subject()
        .await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_subject_by_id(id: String) -> Result<Option<SubjectResDto>, SubjectError> {
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
) -> Result<Vec<SubjectResDto>, SubjectError> {
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