use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

use crate::command_from_dto;
use crate::modules::category::application::dto::ExportCategoryResDto;
use crate::modules::category::domain::CategoryFactory;
use crate::modules::category::domain::CategoryPlainObject;
use crate::modules::category::domain::{Category, CategoryGenericError, CategoryID};
use crate::modules::category::repository::CategoryRepository;
use crate::modules::common::application::ICommandHandler;
use crate::modules::common::domain::ID;
use crate::modules::resource::domain::ResourceFactory;
use crate::modules::resource::domain::ResourceTaggingPlainObject;
use crate::modules::resource::domain::{Resource, ResourcePlainObject};
use crate::modules::resource::repository::ResourceRepository;
use crate::modules::subject::domain::SubjectFactory;
use crate::modules::subject::domain::{SubjectID, Subject, SubjectPlainObject};
use crate::modules::subject::repository::SubjectRepository;
use crate::modules::tag::domain::TagFactory;
use crate::modules::tag::domain::{TagID, Tag, TagPlainObject};
use crate::modules::tag::repository::TagRepository;

mod dto;
pub use dto::*;

#[derive(Deserialize)]
pub struct ImportCategoryCommand {
    new_root_path: String,

    data: String,
}
command_from_dto!(ImportCategoryCommand, ImportCategoryDto);

// =====================================
pub struct ImportCategoryHandler<'a> {
    categroy_repo: &'a CategoryRepository<'a>,
    subject_repo: &'a SubjectRepository<'a>,
    tag_repo: &'a TagRepository<'a>,
    resource_repo: &'a ResourceRepository<'a>,
}

impl<'a> ImportCategoryHandler<'a> {
    pub fn register(
        categroy_repo: &'a CategoryRepository,
        subject_repo: &'a SubjectRepository,
        tag_repo: &'a TagRepository,
        resource_repo: &'a ResourceRepository,
    ) -> Self {
        Self {
            categroy_repo,
            subject_repo,
            tag_repo,
            resource_repo,
        }
    }

    pub fn check_relation(
        &self,
        category: &CategoryPlainObject,
        subjects: &Vec<SubjectPlainObject>,
        tags: &Vec<TagPlainObject>,
        resources: &Vec<ResourcePlainObject>,
    ) -> Result<(), CategoryGenericError> {
        let subject_id_set: HashSet<String> = subjects
            .iter()
            .map(|val| val.id.to_string())
            .collect();

        let tag_id_set: HashSet<String> = tags
            .iter()
            .map(|val| val.id.to_string())
            .collect();

        // check category import rules is exists
        category.rules
            .iter()
            .try_for_each(|val| {
                match tag_id_set.contains(&val.tag_id.to_string()) {
                    true => Ok(()),
                    false => Err(CategoryGenericError::ImportTagIdNotExists())
                }
            })?;

        // check tag's belong subject is exists
        tags.iter().try_for_each(|val| {
            match subject_id_set.contains(&val.belong_subject.to_string()) {
                true => Ok(()),
                false => Err(CategoryGenericError::ImportSubjectIdNotExists())
            }
        })?;

        // check resource's tags is exists
        resources.iter().flat_map(|val| &val.tags).try_for_each(|val| {
            match  tag_id_set.contains(&val.id.to_string()) {
                true => Ok(()),
                false => Err(CategoryGenericError::ImportTagIdNotExists())
            }
        })?;

        Ok(())
    }

    fn create(
        root_path: String,
        category: CategoryPlainObject,
        subjects: Vec<SubjectPlainObject>,
        tags: Vec<TagPlainObject>,
        resources: Vec<ResourcePlainObject>,
    ) -> Result<(Category, Vec<Subject>, Vec<Tag>, Vec<Resource>), Error> {
        // ------------------------------
        // category part
        let category_rules = category.rules.clone(); // add rule later
        let mut new_category = CategoryFactory::from_plain(CategoryPlainObject {
            root_path: root_path.clone(),
            rules: Vec::new(),
            ..category
        })?;
        let category_id = new_category.get_id();
        let new_root = new_category.get_root_path();

        // ------------------------------
        // subject part
        let mut subids: HashMap<String, SubjectID> = HashMap::new();
        let mut new_subjects: Vec<Subject> = Vec::new();

        for subject in subjects {
            let old_id = subject.id.to_string().clone();
            let new_sub = SubjectFactory::from_plain(SubjectPlainObject { 
                belong_category: category_id.clone(), ..subject
            })?;
    
            subids.insert(old_id, new_sub.get_id().clone());

            new_subjects.push(new_sub);
        }

        // ------------------------------
        // tag part
        let mut tagids: HashMap<String, TagID> = HashMap::new();
        let mut new_tags: Vec<Tag> = Vec::new();

        for tag in tags {
            let old_id = tag.id.to_string().clone();
            let new_tag = TagFactory::from_plain(TagPlainObject {
                belong_category: category_id.clone(),
                belong_subject: subids.get(&tag.belong_subject.to_string()).unwrap().clone(),
                ..tag
            })?;
    
            tagids.insert(old_id, new_tag.get_id().clone());

            new_tags.push(new_tag);
        }
        
        // ------------------------------
        // resource part
        let mut new_resources: Vec<Resource> = Vec::new();

        for res in resources {
            let new_tags = res.tags
                .into_iter()
                .map(|val| ResourceTaggingPlainObject { 
                    id: tagids.get(&val.id.to_string()).unwrap().to_owned(), 
                    added_at: val.added_at,
                })
                .collect::<Vec<ResourceTaggingPlainObject>>();
            let new_res = ResourceFactory::from_plain(ResourcePlainObject {
                belong_category: category_id.clone(),
                root_path: new_root.to_string(),
                tags: new_tags,
                ..res
            })?;

            new_resources.push(new_res);
        }

        // ------------------------------
        // category tag mapper re-construct
        for rules in category_rules {
            let tag_id = tagids.get(rules.tag_id.to_str()).unwrap();
            new_category.get_mut_rule_table().add_rule(rules.text, tag_id.clone())?
        }

        Ok((new_category, new_subjects, new_tags, new_resources))
    }
}

#[async_trait]
impl ICommandHandler<ImportCategoryCommand> for ImportCategoryHandler<'_> {

    fn get_name() -> String {
        String::from("Create Category Command")
    }

    type Output = CategoryID;

    async fn execute(&self, command: ImportCategoryCommand) -> Result<Self::Output, Error> {
        let ImportCategoryCommand { 
            new_root_path,
            data,
        } = command;

        let strbyte = BASE64_STANDARD.decode(data)?;
        let str = String::from_utf8(strbyte)?;
        let ExportCategoryResDto {
            category,
            subjects,
            tags,
            resources,
        } = serde_json::from_str(&str)?;

        // check relation is valid
        self.check_relation(&category, &subjects, &tags, &resources)?;

        // create entity
        let (new_category, new_subjects, new_tags, new_resources) = Self::create(new_root_path, category, subjects, tags, resources)?;

        // save category
        let category_id = self.categroy_repo
            .save(new_category)
            .await
            .or(Err(CategoryGenericError::DBInternalError()))?
            .take_id();

        // ------------------------------
        // subject part
        for subject in new_subjects {
            self.subject_repo
                .save(subject)
                .await
                .or(Err(CategoryGenericError::DBInternalError()))?;
        }

        // ------------------------------
        // tag part
        for tag in new_tags {
            self.tag_repo
                .save(tag)
                .await
                .or(Err(CategoryGenericError::DBInternalError()))?;
        }

        // ------------------------------
        // resource part
        for resource in new_resources {
            self.resource_repo
                .save(resource)
                .await
                .or(Err(CategoryGenericError::DBInternalError()))?;
        }

        Ok(category_id)
    }
}
