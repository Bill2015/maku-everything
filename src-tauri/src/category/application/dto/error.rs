use serde::Serialize;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Create category failed")]
    Create(),
    #[error("Updated Category `{0}` failed")]
    Update(String),
    #[error("`{0}` ID is not existed")]
    FindById(String),
    #[error("Get all category failed")]
    FindAll(),
    #[error("unknown Category error")]
    Unknown{ message: String },
}

#[derive(serde::Serialize)]
struct ErrorWrapper {
    error: String,
}
impl Serialize for CategoryError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = match self {
            CategoryError::Create() => "value",
            CategoryError::Update(msg) => msg,
            CategoryError::FindById(msg) => msg,
            CategoryError::FindAll() => "value2",
            CategoryError::Unknown { message } => message
        };
        let wrapper = ErrorWrapper {
            error: error_message.to_string(),
        };
        wrapper.serialize(serializer)
    }
}