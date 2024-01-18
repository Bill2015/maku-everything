use crate::resource::application::query::{SqlObjectData, SQLGroupPrefixType};

pub struct ResourceStringQL {
    qlstr: String,
}

impl ResourceStringQL {
    pub fn get(&self) -> String {
        self.qlstr.clone()
    }
}

impl From<SqlObjectData> for ResourceStringQL {
    fn from(value: SqlObjectData) -> Self {
        let mut q: Vec<String> = Vec::new();

        if value.get_includes().len() > 0 {
            q.push(format!("(<-tagging<-tag.id CONTAINSALL [{}])", value.get_includes().join(", ")));
        }

        if value.get_excludes().len() > 0 {
            q.push(format!("!(<-tagging<-tag.id CONTAINSANY [{}])", value.get_excludes().join(", ")));
        }
        
        if value.get_groups().len() > 0 {
            for group in value.get_groups() {
                match group.prefix {
                    SQLGroupPrefixType::Include => {
                        q.push(format!("(<-tagging<-tag.id CONTAINSANY [{}])", group.items.join(", ")));
                    },
                    SQLGroupPrefixType::Exclude => {
                        q.push(format!("!(<-tagging<-tag.id CONTAINSALL [{}])", group.items.join(", ")));
                    },
                }
            }
        }

        Self { qlstr: q.join(" AND ") }
    }
}