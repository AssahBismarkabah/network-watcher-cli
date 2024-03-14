use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

// Save the results to a file
pub async fn save_to_file(output: String, results: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Open the output file
    let file = OpenOptions::new()
        .create(true)  // Create the file if it does not exist
        .append(true)  // Append to the file if it exists
        .open(output)
        .await?;

    // Create a buffered writer
    let mut writer = BufWriter::new(file);

    // Write all results to the file
    for idx in 0..=results.len() - 1 {
        let result = results.get(idx).unwrap();
        writer.write_all(result.as_bytes()).await?;

        // Add a comma after each result except the last one
        if (idx + 1) < results.len() {
            writer.write_all(b",").await?;
        }
    }

    // Add a newline at the end of the file
    writer.write_all(b"\n").await?;
    writer.flush().await?;

    Ok(())
}