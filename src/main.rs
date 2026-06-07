use clap::Parser;
use fgrep::*;

fn main() {
    let args = Cli::parse();

    Handler::run(args.path, args.pattern, args.ignore_case);
}
