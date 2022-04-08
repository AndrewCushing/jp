use log::{debug, error};
use protobuf::{Message, ProtobufError};
use super::schemas::structs::payment_envelope::Envelope;

pub fn deserialise(bytes: &[u8]) -> Result<Envelope, ProtobufError> {
    let parse_result = Envelope::parse_from_bytes(bytes);
    match parse_result {
        Ok(envelope) => {
            debug!("Managed to get envelope from bytes!");
            Ok(envelope)
        },
        Err(err) => {
            error!("Got an error when trying to get envelope from bytes :(");
            Err(err)
        }
    }
}