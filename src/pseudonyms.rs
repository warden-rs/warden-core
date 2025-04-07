pub mod account {
    tonic::include_proto!("pseudonyms.account");
    /// Account file descriptor
    pub const ACCOUNT_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("account_descriptor");
}

pub mod entity {
    tonic::include_proto!("pseudonyms.entity");
    /// Entity file descriptor
    pub const Entity_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("entity_descriptor");
}

pub mod transaction_relationship {
    tonic::include_proto!("pseudonyms.transaction_relationship");
    /// Transaction Relationship file descriptor
    pub const TRANSACTION_RELATIONSHIP_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("transaction_relationship_descriptor");
}

pub mod account_holder {
    tonic::include_proto!("pseudonyms.account_holder");
    /// Account Holder file descriptor
    pub const ACCOUNT_HOLDER_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("account_holder_descriptor");
}
