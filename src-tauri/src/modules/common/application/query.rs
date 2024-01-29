use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait IQueryHandler<Q> {
    fn get_name() -> String;

    type Output;
    async fn query(&self, query: Q) -> Result<Self::Output, Error>;
}
