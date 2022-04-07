extern crate prost_build;

fn main() {
    prost_build::compile_protos(&[
        "src/payment_envelope.proto",
        "src/payment_envelope_header.proto",
        "src/payment_envelope_body_part.proto",
        "src/payment_form3_data.proto",
        "src/payment_identifiers.proto",
        "src/payment_ledger_posting.proto",
        "src/payment_pacs_008_001_06.proto",
        "src/payment_raw_message_data.proto",
    ], &["src/"]).unwrap();
}