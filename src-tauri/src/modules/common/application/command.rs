use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait ICommandHandler<C> {
    fn get_name() -> String;

    type Output;

    async fn execute(&self, command: C) -> Result<Self::Output, Error>;
}

#[macro_export]
macro_rules! command_from_dto {
    ($command: ty, $dto: ty) => {
        impl From<$dto> for $command {
            fn from(value: $dto) -> Self {
                let v = serde_json::to_value(value).unwrap();
                serde_json::from_value::<$command>(v).unwrap()
            }
        }
    }
}