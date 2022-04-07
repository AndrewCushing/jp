use protobuf::SingularPtrField;
use crate::schemas::structs::payment_envelope::Envelope;
use crate::schemas::structs::payment_envelope_header::{Header, Properties};
use super::schemas::structs::payment_envelope;

pub fn serialise(envelope: Envelope) -> Vec<u8> {
    let mut envelope1: Envelope = payment_envelope::Envelope {
        version: "".to_string(),
        header: Default::default(),
        body: Default::default(),
        unknown_fields: Default::default(),
        cached_size: Default::default()
    };
    envelope1.header = SingularPtrField::from(Option::Some(Header{
        properties: Default::default(),
        signatures: Default::default(),
        state: Default::default(),
        exception: Default::default(),
        unknown_fields: Default::default(),
        cached_size: Default::default()
    }));
    unimplemented!()
}