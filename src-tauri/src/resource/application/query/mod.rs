#[path ="./get-all-resource.rs"]
mod get_all;
pub use get_all::*;

#[path ="./get-by-id-resource.rs"]
mod get_by_id;
pub use get_by_id::*;

#[path ="./resource-detail.rs"]
mod detail;
pub use detail::*;

#[path ="./query-resource.rs"]
mod query;
pub use query::*;
