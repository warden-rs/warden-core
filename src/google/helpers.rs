#[cfg(feature = "time")]
mod time_util;

#[cfg_attr(feature = "time", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "time", serde(untagged))]
/// Date utility
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub(crate) enum DateItem {
    /// string
    String(String),
    /// ts
    Timestamp { seconds: i64, nanos: i32 },
    /// date
    #[cfg(feature = "iso20022")]
    Date { year: i32, month: i32, day: i32 },
}

#[derive(Debug)]
/// Generic JSON value
#[cfg(feature = "serde")]
pub struct GenericParameter(pub(crate) serde_json::Value);

#[cfg(feature = "serde")]
mod impls;
