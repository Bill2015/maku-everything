use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::resource::application::dto::ResourceResDto;
use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::repository::ResourceQueryRepository;
use crate::tag::repository::TagQueryRepository;

mod types;
mod syntax;
use syntax::Syntax;
mod tokenizer;
use tokenizer::Tokenizer;
mod sqlgen;
use sqlgen::SQLQueryGenerator;

pub struct StringResourceQuery {
    pub query_string: String,
}


pub struct StringResourceHandler<'a> {
    resource_repo: &'a ResourceQueryRepository<'a>,
    tag_repo: &'a TagQueryRepository<'a>,
}

impl<'a> StringResourceHandler<'a> {
    pub fn register(resource_repo: &'a ResourceQueryRepository, tag_repo: &'a TagQueryRepository) -> Self {
        Self {
            resource_repo: &resource_repo,
            tag_repo: &tag_repo,
        }
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
        let tokens = tokenizer.parse();

        let mut syntax_checker = Syntax::new(&tokens);
        let res = syntax_checker.check()?;

        let mut sql = SQLQueryGenerator::new(&tokens);
        sql.preprocessor(self.tag_repo).await?;
        sql.gen();

        dbg!("Success");

        Err(ResourceError::Query(ResourceGenericError::Unknown { message: "Good".to_string() }))
        // match result {
        //     Ok(value) => Ok(value),
        //     _ => Err(ResourceError::Query(ResourceGenericError::DBInternalError())),
        // }
    }
}
