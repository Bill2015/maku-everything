use serde::Serialize;
use thiserror;
use anyhow::Error;
use crate::modules::common::domain::ErrorBody;
use crate::serialize_error;

#[derive(thiserror::Error, Debug)]
pub enum TagError {
    #[error("Create Failed")]
    Create(Error),

    #[error("Updated Failed")]
    Update(Error),

    #[error("Retrieve all Tag failed")]
    GetAll(Error),

    #[error("Get specific Tag by id failed")]
    GetById(Error),

    #[error("Query Failed")]
    Query(Error),
}

impl Serialize for TagError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = match self {
            TagError::Create(source) => serialize_error!(self, source),
            TagError::Update(source) => serialize_error!(self, source),
            TagError::GetAll(source) => serialize_error!(self, source),
            TagError::GetById(source) => serialize_error!(self, source),
            TagError::Query(source) => serialize_error!(self, source),
        };
        error_message.serialize(serializer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TagGenericError {
    #[error("Name is empty")]
    NameIsEmpty(),

    #[error("Name is duplicated: {current_name}")]
    NameIsDuplicated{ current_name: String },

    #[error("Description is empty")]
    DescriptionIsEmpty(),

    #[error("Belong category dose not exists")]
    BelongCategoryNotExists(),

    #[error("Belong subject dose not exists")]
    BelongSubjectNotExists(),

    #[error("Id is not exists")]
    IdNotFounded(),

    #[error("Invalid date format")]
    InvalidDateFormat(),

    #[error("unknown Tag error")]
    Unknown{ message: String },
    
    #[error("Database internal error")]
    DBInternalError(),
}

