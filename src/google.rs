/// Well known types
#[cfg(any(feature = "iso20022", feature = "pseudonyms"))]
pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
}

/// Additional types
#[cfg(feature = "iso20022")]
pub mod r#type {
    tonic::include_proto!("google.r#type");
}
