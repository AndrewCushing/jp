use jp::Cli;
use jp::run;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    run(cli);
}