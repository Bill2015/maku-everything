use std::ops::Add;

use crate::impl_query_builder;
use crate::modules::common::infrastructure::{QueryBuilder, BaseQueryBuilder, BaseQueryBuilderError, QueryBuilderResult};
use crate::modules::category::application::query::ListCategoryQuery;

pub struct CategoryQueryBuilder {
    pub base_builder: BaseQueryBuilder,

    pub id: Option<String>,

    pub name: Option<String>,
}

impl_query_builder!(CategoryQueryBuilder, { id, name });

impl CategoryQueryBuilder {
    pub fn new() -> CategoryQueryBuilder {
        CategoryQueryBuilder { 
            base_builder: BaseQueryBuilder::new(),
            id: None,
            name: None,
        }
    }

    fn build_string(&self) -> Result<String, BaseQueryBuilderError> {
        let qdata: Vec<Option<String>> = vec![
            self.id.as_ref()
                .map(|v| format!("id == {}", v).into()),

            self.name.as_ref()
                .map(|v| format!("string::lowercase(name) == string::lowercase(\"{}\")", v)),
        ];
    
        let base_result = self.base_builder.build()?;
        Ok(qdata
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<String>>()
            .join(" AND ")
            .add(" ")
            .add(base_result.to_string().as_str())
        )
    }
}

impl From<ListCategoryQuery> for CategoryQueryBuilder {
    fn from(value: ListCategoryQuery) -> Self {
        let builder = CategoryQueryBuilder {
            base_builder: BaseQueryBuilder::from_value(value.order_by, value.limit, value.start),
            id: value.id,
            name: value.name,
        };

        builder
    }
}