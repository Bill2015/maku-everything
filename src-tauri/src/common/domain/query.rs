use async_trait::async_trait;

#[async_trait]
pub trait QueryHandler<Q> {
    fn get_name() -> String;

    type Output;
    async fn query(query_condition: Q) -> Self::Output; 
}
