use super::{StringQLGroupPrefix, StringQLObject};

/// Generate String Querying Language \
/// This is depend on SurrealDB
pub struct ResourceStringQL {
    qlstr: String,
}

impl ResourceStringQL {
    pub fn get(&self) -> String {
        self.qlstr.clone()
    }
}

impl From<StringQLObject> for ResourceStringQL {
    fn from(value: StringQLObject) -> Self {
        let mut q: Vec<String> = Vec::new();

        // +Typescript +Javascript
        if value.get_includes().len() > 0 {
            q.push(format!("(<-tagging<-tag.id CONTAINSALL [{}])", value.get_includes().join(", ")));
        }

        // -Typescript -Javascript
        if value.get_excludes().len() > 0 {
            q.push(format!("!(<-tagging<-tag.id CONTAINSANY [{}])", value.get_excludes().join(", ")));
        }
        
        // +[Typescript Javascript] =>  ContainAny
        // -[Typescript Javascript] => !ContainAll 
        if value.get_groups().len() > 0 {
            for group in value.get_groups() {
                match group.prefix {
                    StringQLGroupPrefix::Include => {
                        q.push(format!("(<-tagging<-tag.id CONTAINSANY [{}])", group.items.join(", ")));
                    },
                    StringQLGroupPrefix::Exclude => {
                        q.push(format!("!(<-tagging<-tag.id CONTAINSALL [{}])", group.items.join(", ")));
                    },
                }
            }
        }

        Self { qlstr: q.join(" AND ") }
    }
}