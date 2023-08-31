
use crate::category::repository::{CategoryRepository, CATEGORY_REPOSITORY};
use crate::subject::repository::{SubjectRepository, SUBJECT_REPOSITORY};
use crate::tag::repository::{TAG_REPOSITORY, TAG_QUERY_REPOSITORY, TagRepository, TagQueryRepository};
use crate::tag::application::command::{CreateTagCommand, CreateTagHandler};
use crate::common::application::{ICommandHandler, IQueryHandler};

use super::command::{UpdateTagCommand, UpdateTagHandler};
use super::dto::TagResDto;
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
        TagService { 
            category_repository: category_repository,
            subject_repository: subject_repository,
            tag_repository: tag_repository,
            tag_queryrepo: tag_queryrepo,
        }
    }

    pub async fn create_tag(&self, name: String, description: String, belong_category: String, belong_subject: String) -> Result<String, String> {
        // Category
        let category = self.category_repository
            .find_by_id(&belong_category)
            .await
            .unwrap_or(None);

        if category.is_none() {
            println!("Category Not Exist");
            return Err(String::from("Category Not Exist"));
        }
        
        // Subject 
        let subject = self.subject_repository
            .find_by_id(&belong_subject)
            .await
            .unwrap_or(None);

        if subject.is_none() {
            println!("Subject Not Exist");
            return Err(String::from("Category Not Exist"));
        }

        let command = CreateTagCommand {
            name: name,
            description: description,
            belong_category: category.unwrap().id,
            belong_subject: subject.unwrap().id,
        };
        let handler = CreateTagHandler::register(self.tag_repository);
        
        let res = handler.execute(command).await?;

        Ok(res)
    }

    pub async fn update_tag(&self, id: String, name: Option<String>, description: Option<String>, auth: Option<bool>) -> Result<String, String> {
        let command = UpdateTagCommand {
            id: id,
            name: name,
            description: description,
            auth: auth,
        };

        let handler = UpdateTagHandler::register(self.tag_repository);

        let res = handler.execute(command).await?;

        Ok(res)
    }

    pub async fn get_all_tag(&self) -> Result<Vec<TagResDto>, String> {
        let query = GetAllTagQuery { };

        let handler = GetAllTagHandler::register(self.tag_queryrepo);

        let res = handler.query(query).await?;

        Ok(res)
    }

    pub async fn get_tag_by_id(&self, id: String) -> Result<Option<TagResDto>, String> {
        let query = GetByIdTagQuery { id: id };
        
        let handler = GetByIdTagHandler::register(self.tag_queryrepo);

        let res = handler.query(query).await?;

        Ok(res)
    }
}