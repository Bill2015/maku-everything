use serde::{Deserialize, Serialize};

use crate::category::domain::PortingCategoryObject;
use crate::resource::domain::PortingResourceObject;
use crate::subject::domain::PortingSubjectObject;
use crate::tag::domain::PortingTagObject;


#[derive(Serialize, Deserialize)]
pub struct ImportCategoryObjDto(pub PortingCategoryObject);

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryOfSubjectObjDto(pub PortingSubjectObject);

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryOfTagObjDto(pub PortingTagObject);

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryOfResourceObjDto(pub PortingResourceObject);

#[derive(Serialize, Deserialize)]
pub struct ImportCategoryDto {
    pub new_root_path: String,

    pub category: ImportCategoryObjDto,
    
    pub subjects: Vec<ImportCategoryOfSubjectObjDto>,

    pub tags: Vec<ImportCategoryOfTagObjDto>,

    pub resources: Vec<ImportCategoryOfResourceObjDto>,
}