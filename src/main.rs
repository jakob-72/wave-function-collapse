use crate::shared::WfcError;
use rules::Ruleset;
use std::io;
use wfc::Wfc;

mod matrix;
mod rules;
mod shared;
mod vec2i;
mod wfc;

pub type Result<T> = std::result::Result<T, WfcError>;

const DEFAULT_RULESET_FILE: &str = "rules.yaml";

/// change to false for default matrix display
const PRINT_COLORFUL: bool = true;

fn main() -> Result<()> {
    let ruleset = read_ruleset_from_file()?;
    println!("Enter the number of columns: ");
    let cols = get_number_from_input()?;
    println!("Enter the number of rows: ");
    let rows = get_number_from_input()?;

    let mut wfc = Wfc::new(cols, rows, ruleset);
    wfc.run()?;
    wfc.print_matrix(PRINT_COLORFUL);
    Ok(())
}

fn get_number_from_input() -> Result<usize> {
    let input = io::stdin();
    let mut buffer = String::new();
    input.read_line(&mut buffer).expect("Failed to read input");
    Ok(buffer.trim().parse::<usize>()?)
}

fn read_ruleset_from_file() -> Result<Ruleset> {
    let file_content = std::fs::read_to_string(DEFAULT_RULESET_FILE)?;
    Ok(Ruleset::from_yaml(&file_content)?)
}
