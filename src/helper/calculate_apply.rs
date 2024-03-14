use futures::future;

use crate::calculate;

// Apply all operations and return the results
pub async fn apply_operations(operations: Vec<Box<dyn calculate::Calculate>>) -> Vec<String> {
    let mut futures = Vec::new();

    // Create a future for each operation
    for op in operations {
        futures.push(async move { op.calculate().await });
    }

    // Wait for all futures to complete
    future::join_all(futures).await
}