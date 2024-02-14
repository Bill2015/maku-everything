
use anyhow::anyhow;

use crate::modules::category::repository::{CategoryRepository, CATEGORY_REPOSITORY};
use crate::modules::subject::repository::{SubjectRepository, SUBJECT_REPOSITORY};
use crate::modules::tag::domain::{TagError, TagID};
use crate::modules::tag::repository::{TAG_REPOSITORY, TAG_QUERY_REPOSITORY, TagRepository, TagQueryRepository};
use crate::modules::tag::application::command::{CreateTagCommand, CreateTagHandler};
use crate::modules::common::application::{ICommandHandler, IQueryHandler};

use super::command::*;
use super::query::*;
use super::dto::*;

pub static TAG_SERVICE: TagService = TagService::init(
    &CATEGORY_REPOSITORY,
    &SUBJECT_REPOSITORY,
    &TAG_REPOSITORY, 
    &TAG_QUERY_REPOSITORY,
);

pub struct TagService<'a> {
    category_repository: &'a CategoryRepository<'a>,
    subject_repository: &'a SubjectRepository<'a>,
    tag_repository: &'a TagRepository<'a>,
    tag_queryrepo: &'a TagQueryRepository<'a>,
}
impl<'a> TagService<'a> {
    const fn init(
        category_repository: &'a CategoryRepository<'a>,
        subject_repository: &'a SubjectRepository<'a>,
        tag_repository: &'a TagRepository<'a>,
        tag_queryrepo: &'a TagQueryRepository<'a>
    ) -> Self {
        Self { 
            category_repository,
            subject_repository,
            tag_repository,
            tag_queryrepo,
        }
    }

    pub async fn create_tag(&self, data: CreateTagDto) -> Result<TagID, TagError> {
        let command = CreateTagCommand::from(data);
        let result = CreateTagHandler::register(
                self.tag_repository,
                self.category_repository,
                self.subject_repository,
            )
            .execute(command)
            .await
            .map_err(|err| TagError::Create(anyhow!(err)))?;
        
        Ok(result)
    }

    pub async fn update_tag(&self, data: UpdateTagDto) -> Result<TagID, TagError> {
        let command = UpdateTagCommand::from(data);

        let result = UpdateTagHandler::register(self.tag_repository)
            .execute(command)
            .await
            .map_err(|err| TagError::Update(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn get_all_tag(&self) -> Result<Vec<TagResDto>, TagError> {
        let query = GetAllTagQuery { };

        let result = GetAllTagHandler::register(self.tag_queryrepo)
            .query(query).
            await
            .map_err(|err| TagError::GetAll(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn get_tag_by_id(&self, id: String) -> Result<Option<TagResDto>, TagError> {
        let query = GetByIdTagQuery { id: id };
        
        let result = GetByIdTagHandler::register(self.tag_queryrepo)
            .query(query)
            .await
            .map_err(|err| TagError::GetById(anyhow!(err)))?;

        Ok(result)
    }

    pub async fn list_tags(&self, data: QueryTagDto) -> Result<Vec<TagResDto>, TagError> {
        let query = ListTagQuery::from(data);
        
        let result = ListTagHandler::register(self.tag_queryrepo)
            .query(query)
            .await
            .map_err(|err| TagError::Query(anyhow!(err)))?;

        Ok(result)
    }
}