use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::modules::category::domain::CategoryID;
use crate::modules::category::repository::CategoryRepository;
use crate::modules::subject::domain::SubjectID;
use crate::modules::subject::repository::SubjectRepository;
use crate::modules::tag::application::dto::TagAttrDto;
use crate::modules::tag::domain::{TagFactory, TagGenericError, TagID};
use crate::modules::tag::domain::valueobj::TagAttributeFactory;
use crate::modules::tag::repository::TagRepository;
use crate::modules::common::application::ICommandHandler;

mod dto;
pub use dto::*;

#[derive(Deserialize)]
pub struct CreateTagCommand {
    pub name: String,

    pub description: String,

    pub belong_category: String,

    pub belong_subject: String,

    #[serde(flatten)]
    pub attrval: TagAttrDto,
}
command_from_dto!(CreateTagCommand, CreateTagDto);

// =====================================
pub struct CreateTagHandler<'a> {
    tag_repo: &'a TagRepository<'a>,
    category_repo: &'a CategoryRepository<'a>,
    subject_repo: &'a SubjectRepository<'a>,
}

impl<'a> CreateTagHandler<'a> {
    pub fn register(tag_repo: &'a TagRepository, category_repo: &'a CategoryRepository<'a>, subject_repo: &'a SubjectRepository<'a>) -> Self {
        Self { tag_repo, category_repo, subject_repo }
    }
}

#[async_trait]
impl ICommandHandler<CreateTagCommand> for CreateTagHandler<'_> {

    fn get_name() -> String {
        String::from("Create Tag Command")
    }

    type Output = TagID;

    async fn execute(&self, command: CreateTagCommand) -> Result<Self::Output, Error> {
        let CreateTagCommand { 
            name,
            description,
            belong_category,
            belong_subject,
            attrval,
        } = command;

        // get CategoryID
        let category_id = self.category_repo
            .is_exist(&belong_category)
            .await
            .then(|| CategoryID::from(belong_category))
            .ok_or(TagGenericError::BelongCategoryNotExists())?;

        // get SubjectID
        let subject_id = self.subject_repo
            .is_exist(&belong_subject)
            .await
            .then(|| SubjectID::from(belong_subject))
            .ok_or(TagGenericError::BelongSubjectNotExists())?;

        // create tag attribute
        let new_attr = match attrval {
            TagAttrDto::Normal => TagAttributeFactory::create_normal(),
            TagAttrDto::Number { start, end, defval } => TagAttributeFactory::create_number(start, end, defval),
            TagAttrDto::Text { defval } => TagAttributeFactory::create_text(defval),
            TagAttrDto::Date { defval } => TagAttributeFactory::create_date(defval),
            TagAttrDto::Bool { defval } => TagAttributeFactory::create_bool(defval),
        }?;

        // create new tag
        let new_tag = TagFactory::create(name, description, &category_id, &subject_id, new_attr)?;

        // save
        let result = self.tag_repo
            .save(new_tag)
            .await;
        
        match result {
            Ok(value) => Ok(value.take_id()),
            _ => Err(TagGenericError::DBInternalError().into()),
        }
    }
}
