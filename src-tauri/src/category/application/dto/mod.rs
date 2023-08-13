#[path ="./response-category.rs"]
mod response;
pub use response::CategoryResDto;

mod error;
pub use error::CategoryError;
