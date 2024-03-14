use clap::Parser;

use crate::calculate_apply::apply_operations;
use crate::check_internet::CheckInternet;
use crate::get_time::GetTime;
use crate::option::Opt;
use crate::save::save_to_file;

mod option;
mod calculate;
mod check_internet;
mod get_time;
mod save;
mod calculate_apply;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opt = Opt::parse();

    let operations: Vec<Box<dyn calculate::Calculate>> = vec![
        Box::new(GetTime::new()),
        Box::new(CheckInternet::new(opts.url)),
    ];

    // Apply all operations and save the results to a file
    let results = apply_operations(operations).await;

    // Save the results to a file
    save_to_file(opts.output, results).await?;

    Ok(())
}
