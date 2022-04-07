pub fn serialise(envelope: super::payment::Envelope) -> Vec<u8> {
    let g = envelope.header.unwrap();
    let u: String = g.properties.unwrap().message_id;
    unimplemented!()
}