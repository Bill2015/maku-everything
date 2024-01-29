use anyhow::Error;
use async_trait::async_trait;

use crate::modules::common::application::IQueryHandler;
use crate::modules::resource::application::dto::ResourceResDto;
use crate::modules::resource::domain::ResourceGenericError;
use crate::modules::resource::infrastructure::ResourceStringQL;
use crate::modules::resource::repository::ResourceQueryRepository;
use crate::modules::tag::repository::TagQueryRepository;

use self::semantic::StringQLSemantic;
use self::sqlgen::StringQLObjectGenerator;
use self::syntax::StringQLSyntaxChecker;
use self::tokenizer::StringQLTokenizer;

mod types;
mod syntax;
mod tokenizer;
mod sqlgen;
mod token;
mod semantic;

pub struct StringResourceQuery {
    pub query_string: String,
    pub belong_category: Option<String>,
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

    type Output = Vec<ResourceResDto>;

    async fn query(&self, query: StringResourceQuery) -> Result<Self::Output, Error> {
        let StringResourceQuery { query_string, belong_category } = query;

        // get token
        let tokens = StringQLTokenizer::new(&query_string).parse();

        // syntax check
        let _ = StringQLSyntaxChecker::new(&tokens).check()?;
        
        // semantic check
        let new_token = StringQLSemantic::new(&tokens, &belong_category, self.tag_repo).parse().await?;

        // generate QL string
        let sqldata = StringQLObjectGenerator::new(&new_token)
            .set_belong_category(belong_category)
            .gen()?;

        let ql = ResourceStringQL::from(sqldata);

        let result = self.resource_repo.string_ql(ql)
            .await;
        
        match result {
            Ok(value) => Ok(value),
            _ => Err(ResourceGenericError::DBInternalError().into()),
        }
    }
}
