use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::Error;
use async_trait::async_trait;
use serde::Deserialize;

use crate::command_from_dto;
use crate::category::application::dto::import::*;
use crate::category::domain::CategoryAggregate;
use crate::category::domain::CategoryGenericError;
use crate::category::domain::CategoryID;
use crate::category::repository::CategoryRepository;
use crate::common::application::ICommandHandler;
use crate::resource::domain::ResourceAggregate;
use crate::resource::repository::ResourceRepository;
use crate::subject::domain::SubjectAggregate;
use crate::subject::domain::SubjectID;
use crate::subject::repository::SubjectRepository;
use crate::tag::domain::TagAggregate;
use crate::tag::domain::TagID;
use crate::tag::repository::TagRepository;

#[derive(Deserialize)]
pub struct ImportCategoryCommand {
    root_path: String,

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
        let subject_id_set: HashSet<&String> = subjects
            .iter()
            .map(|val| &val.id)
            .collect();

        let tag_id_set: HashSet<&String> = tags
            .iter()
            .map(|val| &val.id)
            .collect();

        // check tag's belong subject is exists
        tags.iter().try_for_each(|val| {
            match subject_id_set.contains(&val.belong_subject) {
                true => Ok(()),
                false => Err(CategoryGenericError::ImportSubjectIdNotExists())
            }
        })?;

        // check resource's tags is exists
        resources.iter().flat_map(|val| &val.tags).try_for_each(|val| {
            match  tag_id_set.contains(&val) {
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
        let mut new_category = CategoryAggregate::new(category.name, category.description, root_path.clone())?;
        new_category.set_created_at(&category.created_at)?;
        new_category.set_updated_at(&category.updated_at)?;


        let category_id = &new_category.id;
        // ------------------------------
        // subject part
        let mut subids: HashMap<String, SubjectID> = HashMap::new();
        let mut new_subjects: Vec<SubjectAggregate> = Vec::new();

        for val in subjects {
            let mut new_sub = SubjectAggregate::new(val.name, val.description, category_id)?;
            new_sub.set_created_at(&val.created_at)?;
            new_sub.set_updated_at(&val.updated_at)?;
    
            subids.insert(val.id, new_sub.id.clone());

            new_subjects.push(new_sub);
        }

        // ------------------------------
        // tag part
        let mut tagids: HashMap<String, TagID> = HashMap::new();
        let mut new_tags: Vec<TagAggregate> = Vec::new();

        for val in tags {
            let belong_subject = subids.get(&val.belong_subject).unwrap();
            let mut new_tag = TagAggregate::new(val.name, val.description, category_id, belong_subject)?;
            new_tag.set_created_at(&val.created_at)?;
            new_tag.set_updated_at(&val.updated_at)?;
    
            tagids.insert(val.id, new_tag.id.clone());

            new_tags.push(new_tag);
        }
        
        // ------------------------------
        // resource part
        let mut new_resources: Vec<ResourceAggregate> = Vec::new();

        for val in resources {
            let mut new_res = ResourceAggregate::new(
                val.name,
                val.description,
                category_id,
                root_path.clone(),
                None,
                val.url,
            )?;
            new_res.set_created_at(&val.created_at)?;
            new_res.set_updated_at(&val.updated_at)?;
    
            for tag in val.tags {
                new_res.add_tag(tagids.get(&tag).unwrap())?;
            }

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
            root_path,
            category,
            subjects,
            tags,
            resources,
        } = command;

        // check relation is valid
        self.check_relation(&subjects, &tags, &resources)?;

        // create entity
        let (new_category, new_subjects, new_tags, new_resources) = Self::create(root_path, category, subjects, tags, resources)?;

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
