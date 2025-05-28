/// Pseudonyms file descriptor
pub const PSEUDONYMS_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("pseudonyms_descriptor");

pub mod account {
    tonic::include_proto!("pseudonyms.account");
}

pub mod entity {
    tonic::include_proto!("pseudonyms.entity");
}

pub mod transaction_relationship {
    tonic::include_proto!("pseudonyms.transaction_relationship");
}

pub mod account_holder {
    tonic::include_proto!("pseudonyms.account_holder");
}
