use std::{future::Future, pin::Pin};

use async_trait::async_trait;

#[async_trait]
pub trait ICommandHandler<C> {
    fn get_name() -> String;

    type Output;
    async fn execute(&self, command: C) -> Self::Output;
}