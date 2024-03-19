// Importing necessary modules
use super::save::save_to_file;

// Testing for successful file writing
#[tokio::test]
async fn test_save_to_file_success() {
    // Prepare test data
    let output = "test_output.txt".to_string();
    let results = vec!["result1".to_string(), "result2".to_string()];

    // Call the save_to_file function
    let result = save_to_file(output.clone(), results.clone()).await;

    // Assert that the result is Ok
    assert!(result.is_ok());

    // Clean up: remove the temporary output file
    std::fs::remove_file(output).unwrap();
}

// Test for file open failure
#[tokio::test]
async fn test_save_to_file_open_failure() {
    // Provide an invalid file path
    let output = "/invalid/file/path.txt".to_string();
    let results = vec!["result1".to_string(), "result2".to_string()];

    // Call the save_to_file function
    let result = save_to_file(output.clone(), results.clone()).await;

    // Assert that the result is an error
    assert!(result.is_err());
}

// Test for write failure
#[tokio::test]
async fn test_save_to_file_write_failure() {
    // Provide a non-writable directory as the output file path
    let output = "non_writable_dir/test_output.txt".to_string();
    let results = vec!["result1".to_string(), "result2".to_string()];

    // Call the save_to_file function
    let result = save_to_file(output.clone(), results.clone()).await;

    // Assert that the result is an error
    assert!(result.is_err());
}
