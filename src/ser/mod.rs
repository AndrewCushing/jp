use protobuf::SingularPtrField;
use crate::schemas::structs::payment_envelope::Envelope;
use crate::schemas::structs::payment_envelope_header::Header;

pub fn serialise(envelope: &mut Envelope) -> Vec<u8> {
    envelope.header = SingularPtrField::from(Option::Some(Header{
        properties: Default::default(),
        signatures: Default::default(),
        state: Default::default(),
        exception: Default::default(),
        unknown_fields: Default::default(),
        cached_size: Default::default()
    }));
    unimplemented!()
}