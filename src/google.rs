/// Well known types
pub mod protobuf {

    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));

    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        derive(serde::Serialize, serde::Deserialize)
    )]
    #[cfg_attr(all(feature = "serde", feature = "time"), serde(untagged))]
    /// Date utility
    #[allow(missing_docs)]
    #[derive(Clone, Debug)]
    pub enum DateItem {
        /// string
        String(String),
        /// ts
        Timestamp { seconds: i64, nanos: i32 },
        /// date
        #[cfg(feature = "iso20022")]
        Date { year: i32, month: i32, day: i32 },
    }

    #[cfg(feature = "time")]
    mod utils {
        use super::{DateItem, Timestamp};

        impl TryFrom<DateItem> for Timestamp {
            type Error = time::Error;

            fn try_from(value: DateItem) -> Result<Self, Self::Error> {
                match value {
                    DateItem::String(ref string) => {
                        <Timestamp as std::str::FromStr>::from_str(string)
                    }
                    #[cfg(feature = "iso20022")]
                    DateItem::Date { year, month, day } => {
                        let date =
                            time::Date::try_from(crate::google::r#type::Date { year, month, day })?;
                        let time = time::Time::MIDNIGHT;
                        let offset = time::UtcOffset::UTC;
                        Ok(date.with_time(time).assume_offset(offset).into())
                    }
                    DateItem::Timestamp { seconds, nanos } => Ok(Self { seconds, nanos }),
                }
            }
        }

        impl From<time::OffsetDateTime> for Timestamp {
            fn from(dt: time::OffsetDateTime) -> Self {
                Timestamp {
                    seconds: dt.unix_timestamp(),
                    nanos: dt.nanosecond() as i32,
                }
            }
        }

        impl From<Timestamp> for String {
            fn from(value: Timestamp) -> Self {
                let odt = time::OffsetDateTime::try_from(value).expect("invalid date");
                odt.format(&time::format_description::well_known::Rfc3339)
                    .expect("format is not rfc3339")
            }
        }

        impl std::str::FromStr for Timestamp {
            type Err = time::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let timestamp =
                    time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)?;

                Ok(Timestamp::from(timestamp))
            }
        }

        impl TryFrom<String> for Timestamp {
            type Error = time::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                <Timestamp as std::str::FromStr>::from_str(&value)
            }
        }

        impl TryFrom<Timestamp> for time::OffsetDateTime {
            type Error = time::Error;

            fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
                let dt = time::OffsetDateTime::from_unix_timestamp(value.seconds)?;

                Ok(dt.replace_nanosecond(value.nanos as u32)?)
            }
        }
    }

    #[cfg(test)]
    #[cfg(feature = "time")]
    mod tests {
        use super::*;
        use time::{Duration, OffsetDateTime};

        #[test]
        fn test_offsetdatetime_to_timestamp() {
            let now = OffsetDateTime::now_utc();
            let timestamp: Timestamp = now.into();

            assert_eq!(timestamp.seconds, now.unix_timestamp());
            assert_eq!(timestamp.nanos, now.nanosecond() as i32);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime() {
            let now = OffsetDateTime::now_utc();
            let timestamp: Timestamp = now.into();
            let dt: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt, now);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_with_nanos() {
            let now = OffsetDateTime::now_utc();
            let nanos = 123456789;
            let dt = now + Duration::nanoseconds(nanos);
            let timestamp: Timestamp = dt.into();
            let dt_from_timestamp: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt_from_timestamp, dt);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_with_negative_nanos() {
            let now = OffsetDateTime::now_utc();
            let nanos = -123456789;
            let dt = now + Duration::nanoseconds(nanos);
            let timestamp: Timestamp = dt.into();
            let dt_from_timestamp: OffsetDateTime = timestamp.try_into().unwrap();

            assert_eq!(dt_from_timestamp, dt);
        }

        #[test]
        fn test_timestamp_to_offsetdatetime_invalid_seconds() {
            let timestamp = Timestamp {
                seconds: i64::MIN,
                nanos: 0,
            };
            let result: Result<OffsetDateTime, time::Error> = timestamp.try_into();
            assert!(result.is_err());
        }
    }
}

/// Additional types
#[cfg(feature = "iso20022")]
pub mod r#type {

    tonic::include_proto!("google.r#type");

    #[cfg(feature = "time")]
    mod utils {
        use crate::google::protobuf::DateItem;

        use super::Date;

        impl From<time::OffsetDateTime> for Date {
            fn from(dt: time::OffsetDateTime) -> Self {
                Self {
                    year: dt.year(),
                    month: dt.month() as i32,
                    day: dt.day() as i32,
                }
            }
        }

        impl From<time::Date> for Date {
            fn from(value: time::Date) -> Self {
                Self {
                    year: value.year(),
                    month: value.month() as i32,
                    day: value.day() as i32,
                }
            }
        }

        impl TryFrom<Date> for time::Date {
            type Error = time::Error;

            fn try_from(value: Date) -> Result<Self, Self::Error> {
                Ok(Self::from_calendar_date(
                    value.year,
                    time::Month::try_from(value.month as u8)?,
                    value.day as u8,
                )?)
            }
        }

        impl std::str::FromStr for Date {
            type Err = time::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let date =
                    time::OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
                        .map(Date::from);

                match date {
                    Ok(dt) => Ok(dt),
                    Err(_e) => {
                        let my_format = time::macros::format_description!("[year]-[month]-[day]");
                        let date = time::Date::parse(&s, &my_format)?;
                        Ok(Date::from(date))
                    }
                }
            }
        }

        impl TryFrom<String> for Date {
            type Error = time::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                <Date as std::str::FromStr>::from_str(&value)
            }
        }

        impl TryFrom<DateItem> for Date {
            type Error = time::Error;

            fn try_from(value: DateItem) -> Result<Self, Self::Error> {
                match value {
                    DateItem::String(ref string) => <Date as std::str::FromStr>::from_str(string),
                    DateItem::Date { year, month, day } => Ok(Date { year, month, day }),
                    DateItem::Timestamp { seconds, nanos } => {
                        let odt =
                            time::OffsetDateTime::try_from(crate::google::protobuf::Timestamp {
                                seconds,
                                nanos,
                            })?;
                        Ok(Self {
                            year: odt.year(),
                            month: odt.month() as i32,
                            day: odt.day() as i32,
                        })
                    }
                }
            }
        }

        impl From<Date> for String {
            fn from(value: Date) -> Self {
                let prepend = |value: i32| -> String {
                    match value.lt(&10) {
                        true => format!("0{}", value),
                        false => value.to_string(),
                    }
                };
                format!(
                    "{}-{}-{}",
                    value.year,
                    prepend(value.month),
                    prepend(value.day),
                )
            }
        }
    }
}

/// Custom types
#[cfg(feature = "configuration")]
pub mod custom {
    tonic::include_proto!("google.custom");

    #[cfg(feature = "serde")]
    pub(crate) mod utils {
        use super::{
            ListValue, NullValue, Struct, Value,
            value::{self, Kind},
        };

        impl TryFrom<serde_json::Value> for value::Kind {
            type Error = String;

            fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
                match value {
                    serde_json::Value::Null => {
                        Ok(value::Kind::NullValue(NullValue::NullValue as i32))
                    }
                    serde_json::Value::Bool(b) => Ok(value::Kind::BoolValue(b)),
                    serde_json::Value::Number(n) => n
                        .as_f64()
                        .map(value::Kind::NumberValue)
                        .ok_or_else(|| "Invalid number".to_string()),
                    serde_json::Value::String(s) => Ok(value::Kind::StringValue(s)),
                    serde_json::Value::Array(arr) => {
                        let values = arr
                            .into_iter()
                            .map(Value::try_from)
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(value::Kind::ListValue(ListValue { values }))
                    }
                    serde_json::Value::Object(map) => {
                        let mut fields = std::collections::HashMap::new();
                        for (k, v) in map {
                            let v = Value::try_from(v)?;
                            fields.insert(k, v);
                        }
                        Ok(value::Kind::StructValue(Struct { fields }))
                    }
                }
            }
        }

        impl TryFrom<serde_json::Value> for Value {
            type Error = String;

            fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
                let kind = Some(value::Kind::try_from(value)?);
                Ok(Value { kind })
            }
        }

        impl From<value::Kind> for serde_json::Value {
            fn from(kind: value::Kind) -> Self {
                match kind {
                    value::Kind::NullValue(_) => serde_json::Value::Null,
                    value::Kind::BoolValue(b) => serde_json::Value::Bool(b),
                    value::Kind::NumberValue(n) => serde_json::Value::Number(
                        serde_json::Number::from_f64(n)
                            .unwrap_or_else(|| serde_json::Number::from(0)),
                    ),
                    value::Kind::StringValue(s) => serde_json::Value::String(s),
                    value::Kind::StructValue(s) => serde_json::Value::from(s),
                    value::Kind::ListValue(l) => serde_json::Value::from(l),
                }
            }
        }

        impl From<Value> for serde_json::Value {
            fn from(value: Value) -> Self {
                match value.kind {
                    Some(kind) => kind.into(),
                    None => serde_json::Value::Null,
                }
            }
        }

        impl From<Struct> for serde_json::Value {
            fn from(s: Struct) -> Self {
                let map = s
                    .fields
                    .into_iter()
                    .map(|(k, v)| (k, serde_json::Value::from(v)))
                    .collect();
                serde_json::Value::Object(map)
            }
        }

        impl From<ListValue> for serde_json::Value {
            fn from(l: ListValue) -> Self {
                let list = l.values.into_iter().map(serde_json::Value::from).collect();
                serde_json::Value::Array(list)
            }
        }

        impl From<Value> for GenericParameter {
            fn from(value: Value) -> Self {
                Self(value.into())
            }
        }

        impl From<Kind> for GenericParameter {
            fn from(value: Kind) -> Self {
                Self(value.into())
            }
        }

        #[derive(Debug)]
        /// Utility JSON type
        pub struct GenericParameter(serde_json::Value);

        impl serde::Serialize for GenericParameter {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let json = serde_json::Value::from(self.0.clone());
                json.serialize(serializer)
            }
        }
    }
}
