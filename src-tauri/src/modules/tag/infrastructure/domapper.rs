use surrealdb::sql::Datetime;

use crate::modules::common::domain::DomainModelMapper;
use crate::modules::tag::domain::TagProps;
use crate::modules::tag::domain::valueobj::TagAttrVO;
use crate::modules::tag::repository::{TagAttrDO, TagDO};

impl DomainModelMapper<TagAttrVO> for TagAttrDO {
    fn to_domain(self) -> TagAttrVO {
        match self {
            Self::Normal => TagAttrVO::Normal,
            Self::WithNumber { start, end, defval } => TagAttrVO::WithNumber { start, end, defval },
            Self::WithText { defval } => TagAttrVO::WithText { defval },
            Self::WithDate { defval } => TagAttrVO::WithDate { defval: defval.0 },
            Self::WithBool { defval } => TagAttrVO::WithBool { defval },
        }
    }

    fn from_domain(value: TagAttrVO) -> Self {
        match value {
            TagAttrVO::Normal => Self::Normal,
            TagAttrVO::WithNumber { start, end, defval } => Self::WithNumber { start, end, defval },
            TagAttrVO::WithText { defval } => Self::WithText { defval },
            TagAttrVO::WithDate { defval } => Self::WithDate { defval: Datetime(defval) },
            TagAttrVO::WithBool { defval } => Self::WithBool { defval },
        }
    }
}

impl DomainModelMapper<TagProps> for TagDO {
    fn to_domain(self) -> TagProps {
        TagProps {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            belong_category: self.belong_category.into(),
            belong_subject: self.belong_subject.into(),
            auth: self.auth,
            attr: self.attrval.to_domain(),
            created_at: self.created_at.0,
            updated_at: self.updated_at.0,
        }
    }
    fn from_domain(value: TagProps) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            belong_category: value.belong_category.into(),
            belong_subject: value.belong_subject.into(),
            auth: value.auth,
            attrval: TagAttrDO::from_domain(value.attr),
            created_at: Datetime(value.created_at),
            updated_at: Datetime(value.updated_at),
        }
    }
}

