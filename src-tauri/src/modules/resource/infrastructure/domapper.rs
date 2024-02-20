use surrealdb::sql::Datetime;

use crate::modules::common::domain::DomainModelMapper;
use crate::modules::resource::domain::entities::ResourceTaggingEntity;
use crate::modules::resource::domain::valueobj::{ResourceFileVO, ResourceTaggingAttrVO, ResourceTaggingVO, ResourceUrlVO};
use crate::modules::resource::domain::ResourceProps;
use crate::modules::resource::repository::{ResourceDO, ResourceFileDo, ResourceTaggingAttrDO, ResourceTaggingDo, ResourceUrlDo};

impl DomainModelMapper<ResourceTaggingAttrVO> for ResourceTaggingAttrDO {
    fn to_domain(self) -> ResourceTaggingAttrVO {
        match self {
            Self::Normal() => ResourceTaggingAttrVO::Normal,
            Self::Number(val) => ResourceTaggingAttrVO::Number(val),
            Self::Text(val) => ResourceTaggingAttrVO::Text(val),
            Self::Date(val) => ResourceTaggingAttrVO::Date(val.0),
            Self::Bool(val) => ResourceTaggingAttrVO::Bool(val),
        }
    }

    fn from_domain(value: ResourceTaggingAttrVO) -> Self {
        match value {
            ResourceTaggingAttrVO::Normal => Self::Normal(),
            ResourceTaggingAttrVO::Number(val) => Self::Number(val),
            ResourceTaggingAttrVO::Text(val) => Self::Text(val),
            ResourceTaggingAttrVO::Date(val) => Self::Date(Datetime(val)),
            ResourceTaggingAttrVO::Bool(val) => Self::Bool(val),
        }
    }
}

impl DomainModelMapper<ResourceTaggingVO> for ResourceTaggingDo {
    fn to_domain(self) -> ResourceTaggingVO {
        ResourceTaggingVO {
            id: self.id.into(),
            added_at: self.added_at.0,
            attrval: self.attrval.to_domain(),
        }
    }
    fn from_domain(value: ResourceTaggingVO) -> Self {
        Self {
            id: value.id.into(),
            added_at: Datetime(value.added_at),
            attrval: ResourceTaggingAttrDO::from_domain(value.attrval),
        }
    }
}

// ====================================================================
impl DomainModelMapper<ResourceFileVO> for ResourceFileDo {
    fn to_domain(self) -> ResourceFileVO {
        let Self { uuid, name, path, ext } = self;
        ResourceFileVO { uuid, name, path, ext }
    }
    fn from_domain(value: ResourceFileVO) -> Self {
        let ResourceFileVO { uuid, name, path, ext  } = value;
        Self { uuid, name, path, ext }
    }
}

// ====================================================================
impl DomainModelMapper<ResourceUrlVO> for ResourceUrlDo {
    fn to_domain(self) -> ResourceUrlVO {
        let Self { host, full } = self;
        ResourceUrlVO { host, full }
    }
    fn from_domain(value: ResourceUrlVO) -> Self {
        let ResourceUrlVO { host, full  } = value;
        Self { host, full }
    }
}

// ====================================================================
impl DomainModelMapper<ResourceProps> for ResourceDO {
    fn to_domain(self) -> ResourceProps {
        let tags: Vec<ResourceTaggingVO> = self.tags
            .into_iter()
            .map(|x| ResourceTaggingDo::to_domain(x.to_owned()) )
            .collect();

        ResourceProps {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            belong_category: self.belong_category.into(),
            file: self.file.map(|val| ResourceFileDo::to_domain(val)),
            url: self.url.map(|val| ResourceUrlDo::to_domain(val)),
            auth: self.auth,
            tagging: ResourceTaggingEntity::new(tags),
            created_at: self.created_at.0,
            updated_at: self.updated_at.0,
        }
    }

    fn from_domain(value: ResourceProps) -> Self {
        let tags = value.tagging
            .vals()
            .into_iter()
            .map(|x| ResourceTaggingDo::from_domain(x.clone()) )
            .collect::<Vec<ResourceTaggingDo>>();

        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            root_path: value.root_path,
            belong_category: value.belong_category.into(),
            file: value.file.map(|val| ResourceFileDo::from_domain(val)),
            url: value.url.map(|val| ResourceUrlDo::from_domain(val)),
            auth: value.auth,
            tags: tags,
            created_at: Datetime(value.created_at),
            updated_at: Datetime(value.updated_at),
        }
   
    }
}
