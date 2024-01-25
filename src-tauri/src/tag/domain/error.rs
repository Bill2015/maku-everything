use serde::Serialize;
use thiserror;
use anyhow::Error;
use crate::common::domain::ErrorBody;


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
            TagError::Create(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            TagError::Update(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            TagError::GetAll(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            TagError::GetById(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            TagError::Query(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            }
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

    #[error("unknown Tag error")]
    Unknown{ message: String },
    
    #[error("Database internal error")]
    DBInternalError(),
}

