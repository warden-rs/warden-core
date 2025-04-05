//! Core
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations
)]
/// Protobuf types
#[cfg(any(
    feature = "configuration",
    feature = "iso20022",
    feature = "pseudonyms"
))]
pub mod google;

/// ISO20022 messages
#[allow(missing_docs)]
#[cfg(feature = "iso20022")]
pub mod iso20022;

/// pseudonyms
#[allow(missing_docs)]
#[cfg(feature = "pseudonyms")]
pub mod pseudonyms;

#[cfg(feature = "configuration")]
/// configuration
pub mod configuration;
