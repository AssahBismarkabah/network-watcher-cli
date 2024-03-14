use async_trait::async_trait;

// This trait is used to define the operations that will be applied to the data
#[async_trait]
pub trait Calculate {
    // This method will be used to calculate the result of the operation
    async fn calculate(&self) -> String;
}
