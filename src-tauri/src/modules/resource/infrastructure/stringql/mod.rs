
mod qlobj;
pub use qlobj::*;

mod attrval;
pub use attrval::*;

mod systags;
pub use systags::SystemTag;

use crate::modules::common::repository::sql_utils;

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

    pub fn item_to_qlstring(item: &StringQLItem) -> String {
        let attribute = item.get_attribute().clone().unwrap_or(AttributeValue::None);
        let value = item.get_value();

        let not_flag = match item.get_prefix() {
            StringQLPrefix::Include => false,
            StringQLPrefix::Exclude => true,
            StringQLPrefix::Inherit => false,
        };

        // system tag
        if *item.is_system() {
            if let Ok(function_tag) = SystemTag::from_str(value, attribute) {
                return sql_utils::sql_with_prefix(not_flag, function_tag.to_qlstring());
            }
            panic!("Invalid SystemTag!");
        }
        // normal
        else {
            let tagging = format!("(<-(tagging WHERE in == {}).attrval)[0]", value);
            let ql = match attribute {
                AttributeValue::None => sql_utils::sql_contain("<-tagging.in", value),
                AttributeValue::Text(text) => sql_utils::sql_contain_string(&tagging, text),
                AttributeValue::OptionText(text) => {
                    match text {
                        Some(val) => sql_utils::sql_contain_string(&tagging, val),
                        None => sql_utils::sql_contain("<-tagging.in", value),
                    }
                },
                AttributeValue::NumberRange(start, end) => sql_utils::sql_range_number(&tagging, (&start, &end)),
                AttributeValue::DateRange(start, end) => sql_utils::sql_range_date(&tagging, (&start, &end)),
                AttributeValue::Bool(val) => sql_utils::sql_equal(&tagging, &val),
            };

            sql_utils::sql_with_prefix(not_flag, ql)
        }
    }
}

impl From<StringQLObject> for ResourceStringQL {
    fn from(qloject: StringQLObject) -> Self {
        let mut q: Vec<String> = Vec::new();

        for item in qloject.get_items() {
            let item_qlstring = ResourceStringQL::item_to_qlstring(&item);
            q.push(item_qlstring);
        }
        
        for group in qloject.get_groups() {
            let group_items: Vec<String> = group.items.iter()
                .map(|item| ResourceStringQL::item_to_qlstring(&item))
                .collect();
            
            // determine the group prefix
            let group_result =  match group.prefix {
                StringQLPrefix::Include => group_items.join(" OR "),
                StringQLPrefix::Exclude =>  group_items.join(" AND "),
                StringQLPrefix::Inherit => group_items.join(" OR "),
            };
            let not_flag = match group.prefix {
                StringQLPrefix::Include => false,
                StringQLPrefix::Exclude => true,
                StringQLPrefix::Inherit => false,
            };

            q.push(sql_utils::sql_with_prefix(not_flag, format!("({})", group_result)));
        }

        Self { qlstr: q.join(" AND ") }
    }
}

