use crate::modules::common::domain::{Porting, ID};
use crate::modules::category::domain::{CategoryAggregate, CategoryGenericError, CategoryID, PortingCategoryObject, PortingRuleItemObject, RuleItemVO, RuleTableEntity};
use crate::modules::common::infrastructure::dateutils;

impl Porting<Vec<PortingRuleItemObject>> for RuleTableEntity {
    type Err = CategoryGenericError;

    fn import_from(data: Vec<PortingRuleItemObject>) -> Result<Self, Self::Err> {
        let rules = data
            .into_iter()
            .map(|val| RuleItemVO {
                text: val.text,
                tag_id: val.tag_id,
            })
            .collect::<Vec<RuleItemVO>>();

        Ok(RuleTableEntity::new(rules))
    }

    fn export_to(self) -> Result<Vec<PortingRuleItemObject>, Self::Err> {
        let rules = self
            .get_rules()
            .into_iter()
            .map(|val| PortingRuleItemObject {
                text: val.text.clone(),
                tag_id: val.tag_id.clone(),
            })
            .collect::<Vec<PortingRuleItemObject>>();

        Ok(rules)
    }
}

impl Porting<PortingCategoryObject> for CategoryAggregate {
    type Err = CategoryGenericError;
    fn import_from(data: PortingCategoryObject) -> Result<Self, Self::Err> {
        let new_path = Self::relove_path(data.root_path)?;
        let rule_table = RuleTableEntity::import_from(data.rule_table)?;
        let category = Self {
            id: CategoryID::new(),
            name: data.name,
            description: data.description,
            root_path: new_path,
            auth: data.auth,
            rule_table: rule_table,
            created_at: dateutils::parse(&data.created_at)
                .map_err(|_| CategoryGenericError::InvalidDateFormat())?
                .and_utc(),
            updated_at: dateutils::parse(&data.updated_at)
                .map_err(|_| CategoryGenericError::InvalidDateFormat())?
                .and_utc(),
        };
        
        Ok(category)
    }

    fn export_to(self) -> Result<PortingCategoryObject, Self::Err> {
        let rule_table = self.rule_table.export_to()?;
        Ok(PortingCategoryObject {
            id: self.id,
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            rule_table: rule_table,
            created_at: dateutils::format(self.created_at),
            updated_at: dateutils::format(self.updated_at),
            auth: self.auth,
        })
    }
}

