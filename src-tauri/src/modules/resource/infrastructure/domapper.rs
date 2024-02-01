use surrealdb::sql::Datetime;

use crate::modules::resource::domain::entities::ResourceTaggingEntity;
use crate::modules::resource::domain::valueobj::{ResourceFileVO, ResourceTaggingVO, ResourceUrlVO};
use crate::modules::resource::domain::ResourceAggregate;
use crate::modules::resource::repository::{ResourceDO, ResourceFileDo, ResourceTaggingDo, ResourceUrlDo};

impl From<ResourceTaggingVO> for ResourceTaggingDo {
    fn from(value: ResourceTaggingVO) -> Self {
        Self {
            id: value.id.into(),
            added_at: Datetime(value.added_at)
        }
    }
}
impl Into<ResourceTaggingVO> for ResourceTaggingDo {
    fn into(self) -> ResourceTaggingVO {
        ResourceTaggingVO {
            id: self.id.into(),
            added_at: self.added_at.0,
        }
    }
}

// ====================================================================
impl From<ResourceFileVO> for ResourceFileDo {
    fn from(value: ResourceFileVO) -> Self {
        Self {
            uuid: value.uuid,
            name: value.name,
            path: value.path,
            ext: value.ext,
        }
    }
}
impl Into<ResourceFileVO> for ResourceFileDo {
    fn into(self) -> ResourceFileVO {
        ResourceFileVO {
            uuid: self.uuid,
            name: self.name,
            path: self.path,
            ext: self.ext,
        }
    }
}

// ====================================================================
impl From<ResourceUrlVO> for ResourceUrlDo {
    fn from(value: ResourceUrlVO) -> Self {
        Self { host: value.host, full: value.full }
    }
}
impl Into<ResourceUrlVO> for ResourceUrlDo {
    fn into(self) -> ResourceUrlVO {
        ResourceUrlVO { host: self.host, full: self.full }
    }
}

// ====================================================================
impl From<ResourceDO> for ResourceAggregate {
    fn from(value: ResourceDO) -> Self {
        let tags = value.tags
            .into_iter()
            .map(|x| x.into() )
            .collect::<Vec<ResourceTaggingVO>>();

        ResourceAggregate {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            root_path: value.root_path,
            belong_category: value.belong_category.into(),
            file: value.file.map(|val| val.into()),
            url: value.url.map(|val| val.into()),
            auth: value.auth,
            tagging: ResourceTaggingEntity::new(tags),
            created_at: value.created_at.0,
            updated_at: value.updated_at.0,
        }
    }
}
impl Into<ResourceDO> for ResourceAggregate {
    fn into(self) -> ResourceDO {
        let tags: Vec<ResourceTaggingDo> = self.tagging
            .vals()
            .into_iter()
            .map(|x| x.to_owned().into() )
            .collect();

        ResourceDO {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            root_path: self.root_path,
            belong_category: self.belong_category.into(),
            file: self.file.map(|val| val.into()),
            url: self.url.map(|val| val.into()),
            auth: self.auth,
            tags: tags,
            created_at: Datetime(self.created_at),
            updated_at: Datetime(self.updated_at),
        }
    }
}