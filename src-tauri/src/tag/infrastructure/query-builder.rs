use std::ops::Add;

use crate::tag::application::query::ListTagQuery;

#[derive(Debug)]
pub struct TagQueryBuilder {
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub belong_subject: Option<String>,

    pub tagging_resource: Option<String>,

    pub order_by: Option<String>,
}

impl TagQueryBuilder {
    pub fn new() -> TagQueryBuilder {
        TagQueryBuilder { 
            id: None,
            name: None,
            belong_category: None,
            belong_subject: None,
            tagging_resource: None,
            order_by: None,
        }
    }

    pub fn set_id(mut self, id: String) -> TagQueryBuilder {
        if !id.is_empty() {
            self.id = Some(format!("id == {}", id));
        }
        self
    }

    pub fn set_name(mut self, name: String) -> TagQueryBuilder {
        if !name.is_empty() {
            self.name = Some(format!("string::lowercase(name) == string::lowercase(\'{}\')", name));
        }
        self
    }

    pub fn set_belong_category(mut self, category_id: String) -> TagQueryBuilder {
        if !category_id.is_empty() {
            self.belong_category = Some(format!("->belong.out CONTAINS '{}'", category_id));
        }
        self
    }

    pub fn set_belong_subject(mut self, subject_id: String) -> TagQueryBuilder {
        if !subject_id.is_empty() {
            self.belong_subject = Some(format!("->belong.out CONTAINS {}", subject_id));
        } 
        self
    }

    pub fn set_tagging_resource(mut self, resource_id: String) -> TagQueryBuilder {
        if !resource_id.is_empty() {
            self.tagging_resource = Some(format!("->tagging.out CONTAINS {}", resource_id));
        }
        self
    }

    pub fn set_order_by(mut self, field_name: String) -> TagQueryBuilder {
        if !field_name.is_empty() {
            self.order_by = Some(format!("ORDER BY {}", field_name));
        }
        self
    }

    pub fn build(&self) -> String {
        let mut query_data: Vec<Option<String>> = Vec::new();
        // NOTE: It seems a code smell
        query_data.push(self.id.to_owned());
        query_data.push(self.name.to_owned());
        query_data.push(self.belong_category.to_owned());
        query_data.push(self.belong_subject.to_owned());
        query_data.push(self.tagging_resource.to_owned());
        
        let query_string: String = query_data
                                .iter()
                                .filter(|x| x.is_some())
                                .map(|v| String::from(v.as_ref().unwrap()))
                                .collect::<Vec<String>>()
                                .join(" AND ");
        
        // order By query string
        if let Some(order_by) = &self.order_by {
            let reuslt = query_string.add(format!(" ORDER BY {}", order_by).as_str());
            return reuslt;
        }
        
        query_string
    }
}

impl From<ListTagQuery> for TagQueryBuilder {
    fn from(value: ListTagQuery) -> Self {
        let mut builder = TagQueryBuilder::new();

        if let Some(id) = value.id {
            builder = builder.set_id(id);
        }

        if let Some(name) = value.name {
            builder = builder.set_name(name);
        }

        if let Some(category_id) = value.belong_category {
            builder = builder.set_belong_category(category_id);
        }

        if let Some(subject_id) = value.belong_subject {
            builder = builder.set_belong_subject(subject_id);
        }

        if let Some(resource_id) = value.tagging_resource {
            builder = builder.set_tagging_resource(resource_id);
        }

        if let Some(field_name) = value.order_by {
            builder = builder.set_order_by(field_name);
        }

        builder
    }
}