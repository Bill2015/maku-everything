use std::ops::Add;

use crate::modules::resource::application::query::ListResourceQuery;

#[derive(Debug)]
pub struct ResourceQueryBuilder {
    pub id: Option<String>,

    pub name: Option<String>,

    pub belong_category: Option<String>, 

    pub exclude_tags: Vec<String>,

    pub include_tags: Vec<String>,

    pub order_by: Option<String>,
}

impl ResourceQueryBuilder {
    pub fn new() -> ResourceQueryBuilder {
        ResourceQueryBuilder { 
            id: None,
            name: None,
            belong_category: None,
            include_tags: Vec::new(),
            exclude_tags: Vec::new(),
            order_by: None,
        }
    }

    pub fn set_id<S: Into<String>>(mut self, id: S) -> ResourceQueryBuilder {
        let id: String = id.into();
        if !id.is_empty() {
            self.id = Some(format!("id == {}", id));
        }
        self
    }

    pub fn set_name(mut self, name: &String) -> ResourceQueryBuilder {
        if !name.is_empty() {
            self.name = Some(format!("string::lowercase(name) == string::lowercase(\'{}\')", name));
        }
        self
    }

    pub fn set_belong_category<S: Into<String>>(mut self, category_id: S) -> ResourceQueryBuilder {
        let category_id: String = category_id.into();
        if !category_id.is_empty() {
            self.belong_category = Some(format!("belong_category == '{}'", category_id));
        }
        self
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

    pub fn set_order_by(mut self, field_name: &String) -> ResourceQueryBuilder {
        if !field_name.is_empty() {
            self.order_by = Some(format!(" ORDER BY {} DESC ", field_name));
        }
        self
    }

    pub fn build(&self) -> String {
        let mut query_data: Vec<Option<String>> = Vec::new();
        // NOTE: It seems a code smell
        query_data.push(self.id.to_owned());
        query_data.push(self.name.to_owned());
        query_data.push(self.belong_category.to_owned());
        
        query_data.push((self.include_tags.len() > 0).then_some(
            format!("(<-tagging<-tag.id) CONTAINSALL [{}]", self.include_tags.join(", "))
        ));
        query_data.push((self.exclude_tags.len() > 0).then_some( 
           format!("!(<-tagging<-tag.id) CONTAINSALL [{}]", self.exclude_tags.join(", "))
        ));
        
        let query_string: String = query_data
                                .iter()
                                .filter(|x| x.is_some())
                                .map(|v| String::from(v.as_ref().unwrap()))
                                .collect::<Vec<String>>()
                                .join(" AND ");
        
        // order By query string
        if let Some(order_by) = &self.order_by {
            let reuslt = query_string.add(order_by.as_str());
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
            builder = builder.set_name(&name);
        }

        if let Some(category_id) = value.belong_category {
            builder = builder.set_belong_category(&category_id);
        }

        if let Some(field_name) = value.order_by {
            builder = builder.set_order_by(&field_name);
        }

        builder
    }
}