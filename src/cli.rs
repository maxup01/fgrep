use clap::Parser;

#[derive(Parser)]
#[command(name = "fgrep", about = "Search file paths by pattern")]
pub struct Cli {
    pattern: String,

    #[arg(default_value = ".")]
    path: String,

    #[arg(short, long)]
    ignore_case: bool,
}
