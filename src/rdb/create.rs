use tonic::Status;

#[tonic::async_trait]
pub trait CreateTable: Sync + Send + 'static {
    async fn create(&self, name: &str) -> Result<(), Status>;
}
