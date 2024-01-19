use std::collections::HashMap;

use async_trait::async_trait;

use crate::common::application::IQueryHandler;
use crate::resource::application::dto::ResourceResDto;
use crate::resource::domain::{ResourceError, ResourceGenericError};
use crate::resource::infrastructure::ResourceStringQL;
use crate::resource::repository::ResourceQueryRepository;
use crate::tag::infrastructure::TagQueryBuilder;
use crate::tag::repository::TagQueryRepository;

mod types;
use types::TokenSymbol;
use types::QueryToken;
mod syntax;
use syntax::Syntax;
mod tokenizer;
use tokenizer::Tokenizer;
mod sqlgen;
use sqlgen::SQLQueryObjectGenerator;

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

        // add EOF symbol
        let q = format!("{}$", query_string.trim());

        // get token
        let mut tokenizer = Tokenizer::new(&q);
        let tokens = tokenizer.parse();

        // syntax check
        let _ = Syntax::new(&tokens).check()?;

        // generate tag id map
        let mut tag_id_map: HashMap<String, String> = HashMap::new();
        for QueryToken { value, namespace, symbol } in &tokens {
            if *symbol != TokenSymbol::TagName {
                continue;
            }
            let mut builder = TagQueryBuilder::new()
                .set_name(value.to_string());

            if let Some(namepace) = namespace {
                builder = builder.set_belong_subject_name(namepace.to_string());
            }

            let result = &self.tag_repo.query(builder)
                .await
                .or(Err(ResourceError::QueryingByString(ResourceGenericError::DBInternalError())))?;

            // find multiple same name tags
            let _ = match result.len() {
                0 => Err(ResourceError::QueryingByString(ResourceGenericError::TagNotExists())),
                1 => {
                    let result = result.first().unwrap();
                    tag_id_map.insert(value.to_string(), result.id.to_string());
                    Ok(())
                },
                _ => Err(ResourceError::QueryingByString(ResourceGenericError::FindAmbiguousTags()))
            }?;
        }

        // generate QL string
        let sql_data = SQLQueryObjectGenerator::new(&tokens, tag_id_map).gen()?;
        let ql = ResourceStringQL::from(sql_data);

        let result = self.resource_repo.string_ql(ql)
            .await;
        
        match result {
            Ok(value) => Ok(value),
            _ => Err(ResourceError::Query(ResourceGenericError::DBInternalError())),
        }
    }
}
