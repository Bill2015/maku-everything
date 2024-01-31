use std::ops::Add;

use crate::impl_query_builder;
use crate::modules::common::infrastructure::{QueryBuilder, BaseQueryBuilder, BaseQueryBuilderError, QueryBuilderResult};
use crate::modules::subject::application::query::ListSubjectQuery;

pub struct SubjectQueryBuilder {
    pub base_builder: BaseQueryBuilder,

    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 
}

impl_query_builder!(SubjectQueryBuilder, { id, name, belong_category });

impl SubjectQueryBuilder {
    pub fn new() -> SubjectQueryBuilder {
        SubjectQueryBuilder { 
            base_builder: BaseQueryBuilder::new(),
            id: None,
            name: None,
            belong_category: None,
        }
    }

    fn build_string(&self) -> Result<String, BaseQueryBuilderError> {
        let qdata: Vec<Option<String>> = vec![
            self.id.as_ref()
                .map(|v| format!("id == {}", v).into()),

            self.name.as_ref()
                .map(|v| format!("string::lowercase(name) == string::lowercase(\"{}\")", v)),

            self.belong_category.as_ref()
                .map(|v| format!("belong_category == \"{}\"", v)),
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

impl From<ListSubjectQuery> for SubjectQueryBuilder {
    fn from(value: ListSubjectQuery) -> Self {
        let builder = SubjectQueryBuilder {
            base_builder: BaseQueryBuilder::from_value(value.order_by, value.limit, value.start),
            id: value.id,
            name: value.name,
            belong_category: value.belong_category,
        };

        builder
    }
}