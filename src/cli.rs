use clap::Parser;

#[derive(Parser)]
#[command(name = "fgrep", about = "Search file paths by pattern")]
pub struct Cli {
    pub pattern: String,

    #[arg(default_value = ".")]
    pub path: String,

    #[arg(short, long)]
    pub ignore_case: bool,
}
