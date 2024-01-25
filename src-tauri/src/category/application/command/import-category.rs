use std::collections::HashMap;
use std::collections::HashSet;

use async_trait::async_trait;
use serde::Deserialize;

use crate::category::application::dto::import::*;
use crate::category::domain::CategoryAggregate;
use crate::category::domain::CategoryError;
use crate::category::domain::CategoryGenericError;
use crate::category::domain::CategoryID;
use crate::category::repository::CategoryRepository;
use crate::common::application::ICommandHandler;
use crate::resource::domain::ResourceAggregate;
use crate::resource::repository::ResourceRepository;
use crate::command_from_dto;
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

    skip_when_resource_not_found: bool,
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
    ) -> Result<(), CategoryError> {
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
                false => Err(CategoryError::Import(CategoryGenericError::ImportSubjectIdNotExists()))
            }
        })?;

        // check resource's tags is exists
        resources.iter().flat_map(|val| &val.tags).try_for_each(|val| {
            match  tag_id_set.contains(&val) {
                true => Ok(()),
                false => Err(CategoryError::Import(CategoryGenericError::ImportSubjectIdNotExists()))
            }
        })?;

        Ok(())
    }
}

#[async_trait]
impl ICommandHandler<ImportCategoryCommand> for ImportCategoryHandler<'_> {

    fn get_name() -> String {
        String::from("Create Category Command")
    }

    type Output = Result<CategoryID, CategoryError>;

    async fn execute(&self, command: ImportCategoryCommand) -> Self::Output {
        let ImportCategoryCommand { 
            root_path,
            category,
            subjects,
            tags,
            resources,
            skip_when_resource_not_found,
        } = command;

        // check relation is valid
        self.check_relation(&subjects, &tags, &resources)?;

        // create new category
        let mut new_category = CategoryAggregate::new(category.name, category.description, root_path.clone())?;

        new_category.set_created_at(&category.created_at);
        new_category.set_updated_at(&category.updated_at);

        // save category
        let category_id = self.categroy_repo
            .save(new_category)
            .await
            .or(Err(CategoryError::Import(CategoryGenericError::DBInternalError())))?
            .id;

        // ------------------------------
        // subject part
        let mut subject_id_hash: HashMap<String, SubjectID> = HashMap::new();

        for subject in subjects {
            // TODO: error handling
            let mut new_subject = SubjectAggregate::new(subject.name, subject.description, category_id.clone()).unwrap();
            
            new_subject.set_created_at(&subject.created_at);
            new_subject.set_updated_at(&subject.updated_at);

            let subject_id = self.subject_repo
                .save(new_subject)
                .await
                .or(Err(CategoryError::Import(CategoryGenericError::DBInternalError())))?
                .id;
    
            subject_id_hash.insert(subject.id, subject_id);
        }

        // ------------------------------
        // tag part
        let mut tag_id_hash: HashMap<String, TagID> = HashMap::new();

        for tag in tags {
            // TODO: error handling
            let mut new_tag = TagAggregate::new(
                tag.name,
                tag.description,
                category_id.clone(),
                subject_id_hash.get(&tag.belong_subject).unwrap().clone()
            ).unwrap();

            new_tag.set_created_at(&tag.created_at);
            new_tag.set_updated_at(&tag.updated_at);

            let tag_id = self.tag_repo
                .save(new_tag)
                .await
                .or(Err(CategoryError::Import(CategoryGenericError::DBInternalError())))?
                .id;

            tag_id_hash.insert(tag.id, tag_id);
        }

        // ------------------------------
        // resource part
        for resource in resources {
            // TODO: error handling
            let mut new_resource = ResourceAggregate::new(
                resource.name,
                resource.description,
                category_id.clone(),
                root_path.clone(),
                None,
                resource.file.clone(),
            ).unwrap();

            if let Some(f) = resource.file {
                let _ = new_resource.change_file(root_path.clone(), f);
            }

            new_resource.set_created_at(&resource.created_at);
            new_resource.set_updated_at(&resource.updated_at);
            
            for tag in resource.tags {
                new_resource.add_tag(tag_id_hash.get(&tag).unwrap().clone());
            }
        }

        Ok(category_id)
    }
}
