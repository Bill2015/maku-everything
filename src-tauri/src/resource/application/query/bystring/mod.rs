use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::resource::application::dto::ResourceResDto;
use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::infrastructure::ResourceStringQL;
use crate::resource::repository::ResourceQueryRepository;
use crate::tag::repository::TagQueryRepository;

mod types;
mod syntax;
use syntax::StringQLSyntaxChecker;
mod tokenizer;
use tokenizer::StringQLTokenizer;
mod sqlgen;
use sqlgen::StringQLObjectGenerator;
mod token;
mod semetic;
use semetic::StringQLSemantic;


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

        // get token
        let tokens = StringQLTokenizer::new(&query_string).parse();

        dbg!(&tokens);

        // syntax check
        let _ = StringQLSyntaxChecker::new(&tokens).check()?;

        // semantic check
        let new_token = StringQLSemantic::new(&tokens, self.tag_repo).parse().await?;

        dbg!(&new_token);

        // generate QL string
        let sql_data = StringQLObjectGenerator::new(&new_token).gen()?;

        dbg!(&sql_data);
        let ql = ResourceStringQL::from(sql_data);

        dbg!(&ql);

        let result = self.resource_repo.string_ql(ql)
            .await;
        
        match result {
            Ok(value) => Ok(value),
            _ => Err(ResourceError::Query(ResourceGenericError::DBInternalError())),
        }
    }
}
