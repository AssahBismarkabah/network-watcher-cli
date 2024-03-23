use anyhow::{Context, Result};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

// Save the results to a file
pub async fn save_to_file(output: String, results: Vec<String>) -> Result<()> {
    // Open the output file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output)
        .await
        .with_context(|| format!("Failed to save: {}", output))?; // Return an error if the file cannot be opened

    // Create a buffered writer
    let mut writer = BufWriter::new(file);

    // Write all results to the file
    for (idx, result) in results.iter().enumerate() {
        writer
            .write_all(result.as_bytes())
            .await
            .with_context(|| format!("Failed to write result to file: {}", output))?; // Return an error if the result cannot be written

        // Add a comma after each result except the last one
        if (idx + 1) < results.len() {
            writer
                .write_all(b",")
                .await
                .with_context(|| format!("Failed to write comma to file: {}", output))?; // Return an error if the comma cannot be written
        }
    }

    // Add a newline at the end of the file
    writer
        .write_all(b"\n")
        .await
        .with_context(|| format!("Failed to write newline to file: {}", output))?;

    // Flush the writer
    writer
        .flush()
        .await
        .with_context(|| format!("Failed to flush writer for file: {}", output))?;

    Ok(())
}
