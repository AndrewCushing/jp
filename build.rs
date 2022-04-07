extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/schemas/structs")
        .inputs(&[
            "src/schemas/proto/payment_envelope.proto",
            "src/schemas/proto/payment_envelope_header.proto",
            "src/schemas/proto/payment_envelope_body_part.proto",
            "src/schemas/proto/payment_form3_data.proto",
            "src/schemas/proto/payment_identifiers.proto",
            "src/schemas/proto/payment_ledger_posting.proto",
            "src/schemas/proto/payment_pacs_008_001_06.proto",
            "src/schemas/proto/payment_raw_message_data.proto",
        ])
        .include("src/schemas/proto")
        .run()
        .expect("Running protoc failed.");
}