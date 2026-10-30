use clap::Parser;
use envctl::cli::Cli;
use envctl::commands::run;
use envctl::error::Result;

fn main() {
    match run_cli() {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(e.exit_code());
        }
    }
}

fn run_cli() -> Result<i32> {
    let cli = Cli::parse();
    run(cli)
}
