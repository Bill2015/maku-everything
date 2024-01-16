use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::resource::application::dto::ResourceResDto;
use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::repository::ResourceQueryRepository;

mod types;
mod tokenizer;
use tokenizer::Tokenizer;

pub struct StringResourceQuery {
    pub query_string: String,
}


pub struct StringResourceHandler<'a> {
    resource_repo: &'a ResourceQueryRepository<'a>,
}

impl<'a> StringResourceHandler<'a> {
    pub fn register(resource_repo: &'a ResourceQueryRepository) -> Self {
        Self { resource_repo: &resource_repo }
    }
}

#[async_trait]
impl IQueryHandler<StringResourceQuery> for StringResourceHandler<'_>{
    fn get_name() -> String {
        String::from("String Querying Resource")
    }

    type Output = Result<Vec<ResourceResDto>, ResourceError>;

    async fn query(&self, query: StringResourceQuery) -> Self::Output {
        let StringResourceQuery { query_string } = query;

        dbg!("========================================");
        let q = format!("{}$", query_string.trim());
        dbg!(&q);

        let mut tokenizer = Tokenizer::new(&q);
        let res = tokenizer.parse();
        dbg!(res);
    
        Err(ResourceError::Query(ResourceGenericError::DBInternalError()))
        // match result {
        //     Ok(value) => Ok(value),
        //     _ => Err(ResourceError::Query(ResourceGenericError::DBInternalError())),
        // }
    }
}
