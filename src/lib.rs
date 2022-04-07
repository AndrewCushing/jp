mod ser;
mod des;

use clap::{Args, Parser, Subcommand};

mod payment {
    include!(concat!(env!("OUT_DIR"), "/protobuf.payment.rs"));
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// Read a payment envelope in json from stdin, or file if specified, then serialise it using protobuf specification.
    s(Serialise),
    /// Deserialise a payment envelope, then convert it to json and send to stdout, or file if specified.
    d(Deserialise),
}

#[derive(Args)]
#[derive(Debug)]
struct Serialise {
    /// File to read payment envelope in json format.
    file_in: Option<String>,
    /// File to write serialised data to (it's ill advised to output binary directly to a terminal).
    file_out: Option<String>,
}

#[derive(Args)]
#[derive(Debug)]
struct Deserialise {
    /// File to read binary from, which should be valid protobuf.
    file_in: Option<String>,
    /// File to write json version of payment envelope to.
    file_out: Option<String>,
}

pub fn run(args: Cli) {
    println!("You have used the subcommand: {:?}", args.command);
}