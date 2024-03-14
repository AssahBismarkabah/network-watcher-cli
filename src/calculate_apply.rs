use futures::future;

use crate::calculate;

pub async fn apply_operations(operations: Vec<Box<dyn calculate::Calculate>>) -> Vec<String> {
    let mut futures = Vec::new();

    for op in operations {
        futures.push(async move { op.calculate().await });
    }

    future::join_all(futures).await
}