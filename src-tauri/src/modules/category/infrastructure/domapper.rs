use surrealdb::sql::Datetime;

use crate::modules::category::domain::{CategoryProps, RuleItemVO, RuleTableEntity};
use crate::modules::category::repository::{CategoryDO, RuleItemDo};
use crate::modules::common::domain::DomainModelMapper;

impl DomainModelMapper<RuleItemVO> for RuleItemDo {
    fn to_domain(self) -> RuleItemVO {
        RuleItemVO {
            text: self.text,
            tag_id: self.tag_id.into(),
        }
    }

    fn from_domain(value: RuleItemVO) -> Self {
        Self {
            text: value.text,
            tag_id: value.tag_id.into(),
        }
    }
}

impl DomainModelMapper<CategoryProps> for CategoryDO {
    fn to_domain(self) -> CategoryProps {
        let rules = self.rules
            .into_iter()
            .map(|x| RuleItemDo::to_domain(x))
            .collect::<Vec<RuleItemVO>>();

        CategoryProps {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            auth: self.auth,
            rule_table: RuleTableEntity::new(rules),
            created_at: self.created_at.0,
            updated_at: self.created_at.0,
        }
    }
    fn from_domain(value: CategoryProps) -> Self {
        let rules: Vec<RuleItemDo> = value.rule_table
            .get_rules()
            .into_iter()
            .map(|x| RuleItemDo::from_domain(x.clone()))
            .collect();

        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            root_path: value.root_path,
            auth: value.auth,
            rules: rules,
            created_at: Datetime(value.created_at),
            updated_at: Datetime(value.updated_at),
        }
    }
}
