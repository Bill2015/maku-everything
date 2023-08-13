
use crate::resource::domain::ResourceAggregate;
use crate::resource::repository::{RESOURSE_REPOSITORY, ResourceRepository};

pub static RESOURSE_SERVICE: ResourceService = ResourceService::init(&RESOURSE_REPOSITORY);

pub struct ResourceService<'a> {
    repository: &'a ResourceRepository<'a>,
}
impl<'a> ResourceService<'a> {
    pub const fn init(repository: &'a ResourceRepository<'_>) -> Self {
        ResourceService { repository: repository }
    }

    pub async fn add_tag(resource_id: String, tag_id: String) {
        // do add tags
    }
}