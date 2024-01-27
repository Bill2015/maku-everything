use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::category::application::dto::import::*;
use crate::category::domain::{CategoryAggregate, CategoryGenericError, CategoryID, PortingCategoryObject};
use crate::category::repository::CategoryRepository;
use crate::common::application::ICommandHandler;
use crate::common::domain::{ID, Porting};
use crate::resource::domain::{ResourceAggregate, PortingResourceObject};
use crate::resource::repository::ResourceRepository;
use crate::subject::domain::{SubjectID, SubjectAggregate, PortingSubjectObject};
use crate::subject::repository::SubjectRepository;
use crate::tag::domain::{TagID, TagAggregate, PortingTagObject};
use crate::tag::repository::TagRepository;

#[derive(Deserialize)]
pub struct ImportCategoryCommand {
    new_root_path: String,

    category: ImportCategoryObjDto,
    
    subjects: Vec<ImportCategoryOfSubjectObjDto>,

    tags: Vec<ImportCategoryOfTagObjDto>,

    resources: Vec<ImportCategoryOfResourceObjDto>,
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
        subjects: &Vec<ImportCategoryOfSubjectObjDto>,
        tags: &Vec<ImportCategoryOfTagObjDto>,
        resources: &Vec<ImportCategoryOfResourceObjDto>,
    ) -> Result<(), CategoryGenericError> {
        let subject_id_set: HashSet<String> = subjects
            .iter()
            .map(|val| val.0.id.to_string())
            .collect();

        let tag_id_set: HashSet<String> = tags
            .iter()
            .map(|val| val.0.id.to_string())
            .collect();

        // check tag's belong subject is exists
        tags.iter().try_for_each(|val| {
            match subject_id_set.contains(&val.0.belong_subject.to_string()) {
                true => Ok(()),
                false => Err(CategoryGenericError::ImportSubjectIdNotExists())
            }
        })?;

        // check resource's tags is exists
        resources.iter().flat_map(|val| &val.0.tags).try_for_each(|val| {
            match  tag_id_set.contains(&val.to_string()) {
                true => Ok(()),
                false => Err(CategoryGenericError::ImportSubjectIdNotExists())
            }
        })?;

        Ok(())
    }

    fn create(
        root_path: String,
        category: ImportCategoryObjDto,
        subjects: Vec<ImportCategoryOfSubjectObjDto>,
        tags: Vec<ImportCategoryOfTagObjDto>,
        resources: Vec<ImportCategoryOfResourceObjDto>,
    ) -> Result<(CategoryAggregate, Vec<SubjectAggregate>, Vec<TagAggregate>, Vec<ResourceAggregate>), Error> {
        let new_category = CategoryAggregate::import_from(PortingCategoryObject {
            root_path: root_path.clone(),
            ..category.0
        })?;
        let category_id = &new_category.id;
        let new_root = &new_category.root_path;

        // ------------------------------
        // subject part
        let mut subids: HashMap<String, SubjectID> = HashMap::new();
        let mut new_subjects: Vec<SubjectAggregate> = Vec::new();

        for ImportCategoryOfSubjectObjDto(subject) in subjects {
            let old_id = subject.id.to_string().clone();
            let new_sub = SubjectAggregate::import_from(PortingSubjectObject { 
                belong_category: category_id.clone(), ..subject
            })?;
    
            subids.insert(old_id, new_sub.id.clone());

            new_subjects.push(new_sub);
        }

        // ------------------------------
        // tag part
        let mut tagids: HashMap<String, TagID> = HashMap::new();
        let mut new_tags: Vec<TagAggregate> = Vec::new();

        for ImportCategoryOfTagObjDto(tag) in tags {
            let old_id = tag.id.to_string().clone();
            let new_tag = TagAggregate::import_from(PortingTagObject {
                belong_category: category_id.clone(),
                belong_subject: subids.get(&tag.belong_subject.to_string()).unwrap().clone(),
                ..tag
            })?;
    
            tagids.insert(old_id, new_tag.id.clone());

            new_tags.push(new_tag);
        }
        
        // ------------------------------
        // resource part
        let mut new_resources: Vec<ResourceAggregate> = Vec::new();

        for ImportCategoryOfResourceObjDto(res) in resources {
            let new_res = ResourceAggregate::import_from(PortingResourceObject {
                belong_category: category_id.clone(),
                root_path: new_root.to_string(),
                ..res
            })?;

            new_resources.push(new_res);
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
            category,
            subjects,
            tags,
            resources,
        } = command;

        // check relation is valid
        self.check_relation(&subjects, &tags, &resources)?;

        // create entity
        let (new_category, new_subjects, new_tags, new_resources) = Self::create(new_root_path, category, subjects, tags, resources)?;

        // save category
        let category_id = self.categroy_repo
            .save(new_category)
            .await
            .or(Err(CategoryGenericError::DBInternalError()))?
            .id;

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
