use std::ops::Add;

use crate::impl_query_builder;
use crate::modules::common::infrastructure::{QueryBuilder, BaseQueryBuilder, BaseQueryBuilderError, QueryBuilderResult};
use crate::modules::tag::application::query::ListTagQuery;

pub struct TagQueryBuilder {
    pub base_builder: BaseQueryBuilder,

    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub belong_subject: Option<String>,

    pub belong_subject_name: Option<String>,

    pub tagging_resource: Option<String>,
}
impl_query_builder!(TagQueryBuilder, { id, name, belong_category, belong_subject, belong_subject_name, tagging_resource });

impl TagQueryBuilder {
    pub fn new() -> TagQueryBuilder {
        TagQueryBuilder {
            base_builder: BaseQueryBuilder::new(),
            id: None,
            name: None,
            belong_category: None,
            belong_subject: None,
            belong_subject_name: None,
            tagging_resource: None,
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

            self.belong_subject.as_ref()
                .map(|v| format!("belong_subject == {}", v)),

            self.belong_subject_name.as_ref()
                .map(|v| format!("string::lowercase(belong_subject.name) == string::lowercase(\"{}\")", v)),

            self.tagging_resource.as_ref()
                .map(|v| format!("->tagging.out CONTAINS {}", v))
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

impl From<ListTagQuery> for TagQueryBuilder {
    fn from(value: ListTagQuery) -> Self {
        let builder = TagQueryBuilder {
            base_builder: BaseQueryBuilder::from_value(value.order_by, value.limit, value.start),
            id: value.id,
            name: value.name,
            belong_category: value.belong_category,
            belong_subject: value.belong_subject,
            belong_subject_name: value.belong_subject_name,
            tagging_resource: value.tagging_resource,
        };

        builder
    }
}