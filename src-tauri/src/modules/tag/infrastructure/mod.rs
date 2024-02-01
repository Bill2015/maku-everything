mod domapper;
pub use domapper::*;

#[path ="./query-builder.rs"]
mod builder;
pub use builder::TagQueryBuilder;
