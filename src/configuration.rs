tonic::include_proto!("configuration");
/// Configuration file descriptor
pub const CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("configuration_descriptor");

pub mod typology {
    tonic::include_proto!("configuration.typology");

    /// Typology Configuration file descriptor
    pub const TYPOLOGY_CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("typology_configuration_descriptor");

    pub mod mathjson {
        tonic::include_proto!("configuration.typology.mathjson");

        /// MathJSON file descriptor
        pub const MATHJSON_CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("mathjson_descriptor");
    }
}

pub mod routing {
    tonic::include_proto!("configuration.routing");

    /// Routing Configuration file descriptor
    pub const ROUTING_CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("routing_configuration_descriptor");
}

pub mod rule {
    tonic::include_proto!("configuration.rule");

    /// Rule Configuration file descriptor
    pub const RULE_CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("rule_configuration_descriptor");
}
