use crate::shared::WfcError;
use clap::Parser;
use cli::Cli;
use rules::Ruleset;
use std::{fs, io};
use wfc::Wfc;

mod cli;
mod matrix;
mod rules;
mod shared;
mod vec2i;
mod wfc;

pub type Result<T> = std::result::Result<T, WfcError>;

/// change to false for default matrix display
const PRINT_COLORFUL: bool = true;

/// change to false to disable saving the output to a file
const SAVE_TO_FILE: bool = true;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let ruleset = read_ruleset_from_file(cli.rules)?;
    let cols = cli.cols.unwrap_or_else(|| {
        println!("Enter the number of columns: ");
        get_number_from_input().expect("Failed to read number of columns")
    });
    let rows = cli.rows.unwrap_or_else(|| {
        println!("Enter the number of rows: ");
        get_number_from_input().expect("Failed to read number of rows")
    });

    let mut wfc = Wfc::new(cols, rows, ruleset);
    wfc.run(SAVE_TO_FILE)?;
    wfc.print_matrix(PRINT_COLORFUL);
    Ok(())
}

fn get_number_from_input() -> Result<usize> {
    let input = io::stdin();
    let mut buffer = String::new();
    input.read_line(&mut buffer).expect("Failed to read input");
    Ok(buffer.trim().parse::<usize>()?)
}

fn read_ruleset_from_file(path: String) -> Result<Ruleset> {
    let file_content = fs::read_to_string(path)?;
    Ok(Ruleset::from_yaml(&file_content)?)
}
