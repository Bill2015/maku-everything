use std::ops::Add;

use crate::resource::application::query::ListResourceQuery;

#[derive(Debug)]
pub struct ResourceQueryBuilder {
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub order_by: Option<String>,
}

impl ResourceQueryBuilder {
    pub fn new() -> ResourceQueryBuilder {
        ResourceQueryBuilder { 
            id: None,
            name: None,
            belong_category: None,
            order_by: None,
        }
    }

    pub fn set_id(mut self, id: String) -> ResourceQueryBuilder {
        if !id.is_empty() {
            self.id = Some(format!("id == {}", id));
        }
        self
    }

    pub fn set_name(mut self, name: String) -> ResourceQueryBuilder {
        if !name.is_empty() {
            self.name = Some(format!("string::lowercase(name) == string::lowercase(\'{}\')", name));
        }
        self
    }

    pub fn set_belong_category(mut self, category_id: String) -> ResourceQueryBuilder {
        if !category_id.is_empty() {
            self.belong_category = Some(format!("belong_category == '{}'", category_id));
        }
        self
    }

    pub fn set_order_by(mut self, field_name: String) -> ResourceQueryBuilder {
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

impl From<ListResourceQuery> for ResourceQueryBuilder {
    fn from(value: ListResourceQuery) -> Self {
        let mut builder = ResourceQueryBuilder::new();

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