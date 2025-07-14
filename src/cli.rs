use clap::Parser;

const DEFAULT_RULESET_FILE: &str = "rules.yaml";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        default_value = DEFAULT_RULESET_FILE,
        help = "Path to the ruleset file, e.g. rules.yaml"
    )]
    pub rules: String,

    #[arg(long, help = "Number of columns in the output matrix")]
    pub cols: Option<usize>,

    #[arg(long, help = "Number of rows in the output matrix")]
    pub rows: Option<usize>,
}
