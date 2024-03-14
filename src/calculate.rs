use async_trait::async_trait;

#[async_trait]
pub trait Calculate {
    async fn calculate(&self) -> String;
}
