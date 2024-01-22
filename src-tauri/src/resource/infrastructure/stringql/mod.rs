
mod qlobj;
pub use qlobj::*;

mod systags;
pub use systags::SystemTag;

/// Generate String Querying Language \
/// This is depend on SurrealDB
#[derive(Debug)]
pub struct ResourceStringQL {
    qlstr: String,
}

impl ResourceStringQL {
    pub fn get(&self) -> String {
        self.qlstr.clone()
    }
}

impl From<StringQLObject> for ResourceStringQL {
    fn from(qloject: StringQLObject) -> Self {
        let mut q: Vec<String> = Vec::new();

        // +Typescript +Javascript
        let mut contains_all: Vec<&str> = Vec::new();

        let mut contains_not: Vec<&str> = Vec::new();

        for item in qloject.get_items() {
            let attribute = item.get_attribute().clone().unwrap_or(AttributeValue::None);
            
            match item.get_prefix() {
                StringQLPrefix::Include => {
                    if let Ok(function_tag) = SystemTag::from_str(item.get_value().as_str(), attribute.clone()) {
                        q.push(function_tag.to_qlstring(false));
                    }
                    else {
                        contains_all.push(item.get_value());
                    }
                },
                StringQLPrefix::Exclude => {
                    if let Ok(function_tag) = SystemTag::from_str(item.get_value().as_str(), attribute.clone()) {
                        q.push(function_tag.to_qlstring(true));
                    }
                    else {
                        contains_not.push(item.get_value());
                    }
                },
                StringQLPrefix::Inherit => {},
            }
        }

        if !contains_all.is_empty() {
            q.push(format!("(<-tagging<-tag.id CONTAINSALL [{}])", contains_all.join(", ")));
        }

        if !contains_not.is_empty() {
            q.push(format!("!(<-tagging<-tag.id CONTAINSANY [{}])", contains_not.join(", ")));
        }
        
        for group in qloject.get_groups() {
            let mut group_items: Vec<String> = Vec::new();
            let mut pure_items: Vec<String> = Vec::new();

            for item in group.items.iter() {
                let attribute = item.get_attribute().clone().unwrap_or(AttributeValue::None);
                if let Ok(function_tag) = SystemTag::from_str(item.get_value().as_str(), attribute.clone()) {
                    group_items.push(function_tag.to_qlstring(false));
                }
                else {
                    pure_items.push(item.get_value().to_string());
                }
            }
            match group.prefix {
                StringQLPrefix::Include => {
                    if !pure_items.is_empty() {
                        group_items.push(format!("(<-tagging<-tag.id CONTAINSANY [{}])", pure_items.join(", ")));
                    }
                    q.push(format!("({})", group_items.join(" OR ")))
                },
                StringQLPrefix::Exclude => {
                    if !pure_items.is_empty() {
                        group_items.push(format!("(<-tagging<-tag.id CONTAINSALL [{}])", pure_items.join(", ")));
                    }
                    q.push(format!("!({})", group_items.join(" AND ")))
                },
                StringQLPrefix::Inherit => {}
            }
        }

        Self { qlstr: q.join(" AND ") }
    }
}