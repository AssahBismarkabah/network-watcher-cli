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
    let opts: Opt = Opt::parse();

    let operations: Vec<Box<dyn calculate::Calculate>> = vec![
        Box::new(GetTime::new()),
        Box::new(GetWifiName::new()),
        Box::new(CheckInternet::new(opts.url)),
        // Add more operations here
    ];

    // Apply all operations and save the results to a file
    let results = apply_operations(operations).await;

    // Save the results to a file
    save_to_file(opts.output, results).await?;

    Ok(())
}
