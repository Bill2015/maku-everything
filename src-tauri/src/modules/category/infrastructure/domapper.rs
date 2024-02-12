use surrealdb::sql::Datetime;

use crate::modules::category::domain::{CategoryAggregate, RuleItemVO, RuleTableEntity};
use crate::modules::category::repository::{CategoryDO, RuleItemDo};

impl From<RuleItemVO> for RuleItemDo {
    fn from(value: RuleItemVO) -> Self {
        Self {
            text: value.text,
            tag_id: value.tag_id.into(),
        }
    }
}

impl Into<RuleItemVO> for RuleItemDo {
    fn into(self) -> RuleItemVO {
        RuleItemVO {
            text: self.text,
            tag_id: self.tag_id.into(),
        }
    }
}

impl From<CategoryDO> for CategoryAggregate {
    fn from(value: CategoryDO) -> Self {
        let rules = value.rules
            .into_iter()
            .map(|x| x.into() )
            .collect::<Vec<RuleItemVO>>();

        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            root_path: value.root_path,
            auth: value.auth,
            rule_table: RuleTableEntity::new(rules),
            created_at: value.created_at.0,
            updated_at: value.created_at.0,
        }
    }
}
impl Into<CategoryDO> for CategoryAggregate {
    fn into(self) -> CategoryDO {
        let rules: Vec<RuleItemDo> = self.rule_table
            .get_rules()
            .into_iter()
            .map(|x| x.to_owned().into() )
            .collect();

        CategoryDO {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            auth: self.auth,
            rules: rules,
            created_at: Datetime(self.created_at),
            updated_at: Datetime(self.updated_at),
        }
    }
}