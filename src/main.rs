use clap::Parser;

use crate::helper::calculate_apply::apply_operations;
use crate::helper::save::save_to_file;
use crate::operations::calculate;
use crate::operations::check_internet::CheckInternet;
use crate::operations::get_time::GetTime;
use crate::operations::get_wifi_name::GetWifiName;
use crate::option::Opt;

mod option;
mod helper;
mod operations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let opts: Opt = Opt::parse();

    // Create operations to collect network data
    let operations: Vec<Box<dyn calculate::Calculate>> = vec![
        Box::new(GetTime::new()),
        Box::new(GetWifiName::new()),
        Box::new(CheckInternet::new(opts.url.clone())), // Pass URL from command-line arguments
        // Add more operations here
    ];

    // Apply all operations and collect the results
    let results = apply_operations(operations).await;

    // Save the results to a file
    match save_to_file(opts.output.clone(), results).await {
        Ok(_) => println!("Results saved successfully to {}", opts.output),
        Err(err) => eprintln!("Failed to save results: {}", err),
    }

    Ok(())
}
