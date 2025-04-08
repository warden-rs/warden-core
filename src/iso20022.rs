/// pacs.008.001.12
pub mod pacs008 {
    tonic::include_proto!("iso20022.pacs008");

    /// Pacs008 file descriptor
    pub const PACS008_CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("pacs008_descriptor");
}

/// pacs.002.001.12
pub mod pacs002 {
    tonic::include_proto!("iso20022.pacs002");
    /// Pacs002 file descriptor
    pub const PACS002_CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("pacs002_descriptor");
}

pub mod payload {
    tonic::include_proto!("iso20022.payload");
}
