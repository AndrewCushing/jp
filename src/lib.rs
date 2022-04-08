mod schemas;
mod ser;
mod des;

use clap::{Args, Parser, Subcommand};
use log::{error, info};
use protobuf::ProtobufError;
use crate::schemas::structs::payment_envelope::Envelope;

pub fn deserialise(bytes: &[u8]) -> Option<Envelope> {
    match des::deserialise(bytes) {
        Ok(envelope) => {
            Some(envelope)
        }
        Err(err) => {
            error!("Something went wrong when deserialising: {}", err);
            None
        }
    }
}