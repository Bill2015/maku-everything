use serde::Serialize;
use thiserror;
use anyhow::Error;

use crate::common::domain::ErrorBody;
use crate::serialize_error;

#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Create Failed")]
    Create(Error),

    #[error("Updated Failed")]
    Update(Error),

    #[error("Retrieve all Category failed")]
    GetAll(Error),

    #[error("Get specific Category by id failed")]
    GetById(Error),

    #[error("Import category failed")]
    Import(Error),
}

impl Serialize for CategoryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = match self {
            CategoryError::Create(source) => serialize_error!(self, source),
            CategoryError::Update(source) => serialize_error!(self, source),
            CategoryError::GetAll(source) => serialize_error!(self, source),
            CategoryError::GetById(source) => serialize_error!(self, source),
            CategoryError::Import(source) => serialize_error!(self, source),
        };
        error_message.serialize(serializer)
    }
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

    #[error("Import category id not exists")]
    ImportCategoryIdNotExists(),

    #[error("Import subject id not exists")]
    ImportSubjectIdNotExists(),

    #[error("Import tag id not exists")]
    ImportTagIdNotExists(),

    #[error("unknown Category error")]
    Unknown{ message: String },

    #[error("Database internal error")]
    DBInternalError(),
}