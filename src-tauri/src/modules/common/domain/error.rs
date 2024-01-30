#[macro_export]
macro_rules! serialize_error {
    ($self:ident, $source:ident) => {
        ErrorBody {
            message: $source.to_string(),
            command: $self.to_string(),
        }
    };
}


#[derive(serde::Serialize)]
pub struct ErrorBody {
    pub message: String,
    pub command: String,
}
