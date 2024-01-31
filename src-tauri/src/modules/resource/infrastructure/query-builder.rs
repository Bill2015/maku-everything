use std::ops::Add;

use crate::impl_query_builder;
use crate::modules::resource::application::query::ListResourceQuery;
use crate::modules::common::infrastructure::{BaseQueryBuilder, BaseQueryBuilderError, QueryBuilder, QueryBuilderResult};

pub struct ResourceQueryBuilder {
    pub base_builder: BaseQueryBuilder,

    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub exclude_tags: Vec<String>,

    pub include_tags: Vec<String>,
}

impl_query_builder!(ResourceQueryBuilder, { id, name, belong_category });

impl ResourceQueryBuilder {
    pub fn new() -> ResourceQueryBuilder {
        ResourceQueryBuilder { 
            base_builder: BaseQueryBuilder::new(),
            id: None,
            name: None,
            belong_category: None,
            include_tags: Vec::new(),
            exclude_tags: Vec::new(),
        }
    }

    pub fn add_include_tag<S: Into<String>>(mut self, tag_id: S) -> ResourceQueryBuilder {
        let tag_id: String = tag_id.into();
        if !tag_id.is_empty() {
            self.include_tags.push(tag_id.to_string());
        }
        self
    }

    pub fn add_exclude_tag<S: Into<String>>(mut self, tag_id: S) -> ResourceQueryBuilder {
        let tag_id: String = tag_id.into();
        if !tag_id.is_empty() {
            self.exclude_tags.push(tag_id.to_string());
        }
        self
    }

    fn build_string(&self) -> Result<String, BaseQueryBuilderError> {
        let qdata: Vec<Option<String>> = vec![
            self.id.as_ref()
                .map(|v| format!("id == {}", v).into()),

            self.name.as_ref()
                .map(|v| format!("string::lowercase(name) == string::lowercase(\"{}\")", v)),
    
            self.belong_category.as_ref()
                .map(|v| format!("belong_category == \"{}\"", v)),
    
            (!self.exclude_tags.is_empty()).then_some(
                format!("!(<-tagging<-tag.id) CONTAINSALL [{}]", self.exclude_tags.join(", "))
            ),
            (!self.include_tags.is_empty()).then_some(
                format!("(<-tagging<-tag.id) CONTAINSALL [{}]", self.include_tags.join(", "))
            )
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

impl From<ListResourceQuery> for ResourceQueryBuilder {
    fn from(value: ListResourceQuery) -> Self {
        let builder = ResourceQueryBuilder {
            base_builder: BaseQueryBuilder::from_value(value.order_by, value.limit, value.start),
            id: value.id,
            name: value.name,
            belong_category: value.belong_category,
            // TODO: ...
            exclude_tags: Vec::new(),
            include_tags: Vec::new(),
        };

        builder
    }
}