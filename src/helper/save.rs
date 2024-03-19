#[allow(unused_imports)]
use std::io;

use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

// Save the results to a file
pub async fn save_to_file(output: String, results: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Open the output file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output)
        .await
        .map_err(|err| {
            format!("Failed to open file '{}': {}", output, err)
        })?;

    // Create a buffered writer
    let mut writer = BufWriter::new(file);

    // Write all results to the file
    for (idx, result) in results.iter().enumerate() {
        writer.write_all(result.as_bytes())
            .await
            .map_err(|err| {
                format!("Failed to write result {}: {}", idx, err)
            })?;

        // Add a comma after each result except the last one
        if (idx + 1) < results.len() {
            writer.write_all(b",")
                .await
                .map_err(|err| {
                    format!("Failed to write comma after result {}: {}", idx, err)
                })?;
        }
    }

    // Add a newline at the end of the file
    writer.write_all(b"\n")
        .await
        .map_err(|err| {
            format!("Failed to write newline character: {}", err)
        })?;

    // Flush the writer
    writer.flush()
        .await
        .map_err(|err| {
            format!("Failed to flush writer: {}", err)
        })?;

    Ok(())
}
