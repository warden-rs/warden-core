/// Well known types
#[cfg(feature = "iso20022")]
pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
}

/// Additional types
#[cfg(feature = "iso20022")]
pub mod r#type {
    tonic::include_proto!("google.r#type");
}
