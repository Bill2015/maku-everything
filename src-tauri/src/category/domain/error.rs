use serde::Serialize;
use thiserror;

use crate::common::domain::ErrorBody;


#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Create Failed")]
    Create(#[source] CategoryGenericError),

    #[error("Updated Failed")]
    Update(#[source] CategoryGenericError),

    #[error("Retrieve all Category failed")]
    GetAll(#[source] CategoryGenericError),

    #[error("Get specific Category by id failed")]
    GetById(#[source] CategoryGenericError),
}

#[derive(thiserror::Error, Debug)]
pub enum CategoryGenericError {
    #[error("Name is empty")]
    NameIsEmpty(),

    #[error("Name is duplicated: {current_name}")]
    NameIsDuplicated{ current_name: String },

    #[error("Description is empty")]
    DescriptionIsEmpty(),

    #[error("Root path is empty")]
    RootPathIsEmpty(),

    #[error("Root path is not exists")]
    RootPathNotExists(),

    #[error("Id is not exists")]
    IdNotFounded(),

    #[error("unknown Category error")]
    Unknown{ message: String },
}

impl Serialize for CategoryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = match self {
            CategoryError::Create(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            CategoryError::Update(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            CategoryError::GetAll(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            CategoryError::GetById(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            }
        };
        error_message.serialize(serializer)
    }
}