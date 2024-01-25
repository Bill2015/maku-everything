use serde::Serialize;
use thiserror;
use anyhow::Error;

use crate::common::domain::ErrorBody;


#[derive(thiserror::Error, Debug)]
pub enum SubjectError {
    #[error("Create Failed")]
    Create(Error),

    #[error("Updated Failed")]
    Update(Error),

    #[error("Retrieve all Subject failed")]
    GetAll(Error),

    #[error("Get specific Subject by id failed")]
    GetById(Error),

    #[error("Query Failed")]
    Query(Error),
}


impl Serialize for SubjectError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = match self {
            SubjectError::Create(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            SubjectError::Update(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            SubjectError::GetAll(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            SubjectError::GetById(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            SubjectError::Query(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            }
        };
        error_message.serialize(serializer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SubjectGenericError {
    #[error("Name is empty")]
    NameIsEmpty(),

    #[error("Name is duplicated: {current_name}")]
    NameIsDuplicated{ current_name: String },

    #[error("Description is empty")]
    DescriptionIsEmpty(),

    #[error("Belong category dose not exists")]
    BelongCategoryNotExists(),

    #[error("Id is not exists")]
    IdNotFounded(),

    #[error("unknown Subject error")]
    Unknown{ message: String },

    #[error("Database internal error")]
    DBInternalError(),
}
