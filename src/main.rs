use jp::Cli;
use jp::run;
use clap::Parser;

mod payment_envelope {
    include!(concat!(env!("OUT_DIR"), "/protobuf.payment.rs"));
}

fn main() {
    let cli = Cli::parse();
    run(cli);
}