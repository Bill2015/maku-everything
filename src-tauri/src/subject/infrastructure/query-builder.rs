use std::ops::Add;

use crate::subject::application::query::ListSubjectQuery;

#[derive(Debug)]
pub struct SubjectQueryBuilder {
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub order_by: Option<String>,
}

impl SubjectQueryBuilder {
    pub fn new() -> SubjectQueryBuilder {
        SubjectQueryBuilder { 
            id: None,
            name: None,
            belong_category: None,
            order_by: None,
        }
    }

    pub fn set_id(mut self, id: String) -> SubjectQueryBuilder {
        if !id.is_empty() {
            self.id = Some(format!("id == {}", id));
        }
        self
    }

    pub fn set_name(mut self, name: String) -> SubjectQueryBuilder {
        if !name.is_empty() {
            self.name = Some(format!("string::lowercase(name) == string::lowercase(\'{}\')", name));
        }
        self
    }

    pub fn set_belong_category(mut self, category_id: String) -> SubjectQueryBuilder {
        if !category_id.is_empty() {
            self.belong_category = Some(format!("belong_category == '{}'", category_id));
        }
        self
    }

    pub fn set_order_by(mut self, field_name: String) -> SubjectQueryBuilder {
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

impl From<ListSubjectQuery> for SubjectQueryBuilder {
    fn from(value: ListSubjectQuery) -> Self {
        let mut builder = SubjectQueryBuilder::new();

        if let Some(id) = value.id {
            builder = builder.set_id(id);
        }

        if let Some(name) = value.name {
            builder = builder.set_name(name);
        }

        if let Some(category_id) = value.belong_category {
            builder = builder.set_belong_category(category_id);
        }

        if let Some(field_name) = value.order_by {
            builder = builder.set_order_by(field_name);
        }

        builder
    }
}