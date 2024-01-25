use serde::Serialize;
use thiserror;
use anyhow::Error;
use crate::common::domain::ErrorBody;


#[derive(thiserror::Error, Debug)]
pub enum ResourceError {
    #[error("Create Failed")]
    Create(Error),

    #[error("Updated Failed")]
    Update(Error),

    #[error("Retrieve all Resource failed")]
    GetAll(Error),

    #[error("Get specific Resource by id failed")]
    GetById(Error),

    #[error("Get Resource Detail by Id failed")]
    Detail(Error),

    #[error("Query Resource failed")]
    Query(Error),

    #[error("Querying by string failed")]
    QueryingByString(Error),

    #[error("Add tag")]
    AddTag(Error),
    
    #[error("Remove tag")]
    RemoveTag(Error),

    #[error("Explore file failed")]
    ExploreFile(Error),
}

impl Serialize for ResourceError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = match self {
            ResourceError::Create(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::Update(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::GetAll(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::GetById(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::Detail(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::Query(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::QueryingByString(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::AddTag(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::RemoveTag(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
            ResourceError::ExploreFile(source) => ErrorBody {
                message: source.to_string(),
                command: self.to_string(),
            },
        };
        error_message.serialize(serializer)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ResourceGenericError {
    #[error("Name and file path is empty")]
    NameAndFilePathIsEmpty(),

    #[error("URL parse failed")]
    UrlParseFailed(),

    #[error("URL empty host")]
    UrlEmptyHost(),

    #[error("File path is not Exist")]
    FilePathNotExist(),

    #[error("No File name")]
    FileNameIsEmpty(),
    
    #[error("Description is empty")]
    DescriptionIsEmpty(),

    #[error("Can't not add same tag")]
    AddSameTag(),

    #[error("Tag that dose not exists")]
    TagNotExists(),

    #[error("Find the ambiguous tags")]
    FindAmbiguousTags(),

    #[error("Id not found")]
    IdNotFound(),

    #[error("Belong Category id is not exists")]
    BelongCategoryNotExists(),

    #[error("Invalid querying string: {message}")]
    InvalidQueryingString{ message: String },

    #[error("unknown Category error")]
    Unknown{ message: String },

    #[error("Database internal error")]
    DBInternalError(),
}