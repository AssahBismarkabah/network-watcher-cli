use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

pub async fn save_to_file(output: String, results: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Open the output file
    let file = File::create(output).await?;
    let mut writer = BufWriter::new(file);

    // Write all results to the file
    for result in results {
        writer.write_all(result.as_bytes()).await?;
        writer.write_all(b",").await?;
    }

    writer.flush().await?;

    Ok(())
}