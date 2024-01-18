
use crate::category::repository::{CategoryRepository, CATEGORY_REPOSITORY};
use crate::subject::repository::{SubjectRepository, SUBJECT_REPOSITORY};
use crate::tag::domain::TagError;
use crate::tag::repository::{TAG_REPOSITORY, TAG_QUERY_REPOSITORY, TagRepository, TagQueryRepository};
use crate::tag::application::command::{CreateTagCommand, CreateTagHandler};
use crate::common::application::{ICommandHandler, IQueryHandler};

use super::command::{UpdateTagCommand, UpdateTagHandler};
use super::dto::{TagResDto, CreateTagDto, UpdateTagDto};
use super::query::*;

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

    pub async fn create_tag(&self, data: CreateTagDto) -> Result<String, TagError> {
        let command = CreateTagCommand::from(data);
        let result = CreateTagHandler::register(
                self.tag_repository,
                self.category_repository,
                self.subject_repository,
            )
            .execute(command)
            .await?;
        
        Ok(result)
    }

    pub async fn update_tag(&self, data: UpdateTagDto) -> Result<String, TagError> {
        let command = UpdateTagCommand::from(data);

        let result = UpdateTagHandler::register(self.tag_repository)
            .execute(command)
            .await?;

        Ok(result)
    }

    pub async fn get_all_tag(&self) -> Result<Vec<TagResDto>, TagError> {
        let query = GetAllTagQuery { };

        let result = GetAllTagHandler::register(self.tag_queryrepo)
            .query(query).
            await?;

        Ok(result)
    }

    pub async fn get_tag_by_id(&self, id: String) -> Result<Option<TagResDto>, TagError> {
        let query = GetByIdTagQuery { id: id };
        
        let result = GetByIdTagHandler::register(self.tag_queryrepo)
            .query(query)
            .await?;

        Ok(result)
    }

    pub async fn list_tags(
        &self, 
        id: Option<String>,
        name: Option<String>,
        belong_category: Option<String>, 
        belong_subject: Option<String>,
        belong_subject_name: Option<String>,
        tagging_resource: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<TagResDto>, TagError> {
        let query = ListTagQuery { 
            id,
            name,
            belong_category,
            belong_subject,
            belong_subject_name,
            tagging_resource,
            order_by
        };
        
        let result = ListTagHandler::register(self.tag_queryrepo)
            .query(query)
            .await?;

        Ok(result)
    }
}