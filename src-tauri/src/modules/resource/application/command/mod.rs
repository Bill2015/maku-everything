#[path="./create-resource.rs"]
mod create;
pub use create::*;

#[path="./update-resource.rs"]
mod update;
pub use update::*;

#[path ="./resource-add-tag.rs"]
mod add_tag;
pub use add_tag::*;

#[path ="./resource-remove-tag.rs"]
mod remove_tag;
pub use remove_tag::*;
