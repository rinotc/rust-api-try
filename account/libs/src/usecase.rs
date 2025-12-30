use async_trait::async_trait;

#[async_trait]
pub trait UseCase<I: Input<O>, O: Output>: Send + Sync {
    async fn execute(&self, input: I) -> O;
}

pub trait Input<O: Output> {}

pub trait Output {}