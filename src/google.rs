/// Well known types
pub mod protobuf {

    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));

    #[cfg_attr(
        all(feature = "serde", feature = "time"),
        derive(serde::Serialize, serde::Deserialize, Clone, Debug)
    )]
    #[cfg_attr(all(feature = "serde", feature = "time"), serde(untagged))]
    /// Date utility
    #[allow(missing_docs)]
    pub enum DateItem {
        /// string
        String(String),
        /// ts
        Timestamp { seconds: i64, nanos: i32 },
        /// date
        Date { year: i32, month: i32, day: i32 },
    }

    #[cfg(all(feature = "serde", feature = "time"))]
    impl TryFrom<DateItem> for Timestamp {
        type Error = time::Error;

        fn try_from(value: DateItem) -> Result<Self, Self::Error> {
            match value {
                DateItem::String(ref string) => <Timestamp as std::str::FromStr>::from_str(string),
                DateItem::Date { year, month, day } => {
                    let date = time::Date::try_from(super::r#type::Date { year, month, day })?;
                    let time = time::Time::MIDNIGHT;
                    let offset = time::UtcOffset::UTC;
                    Ok(date.with_time(time).assume_offset(offset).into())
                }
                DateItem::Timestamp { seconds, nanos } => Ok(Self { seconds, nanos }),
            }
        }
    }

    #[cfg(feature = "time")]
    impl From<time::OffsetDateTime> for Timestamp {
        fn from(dt: time::OffsetDateTime) -> Self {
            Timestamp {
                seconds: dt.unix_timestamp(),
                nanos: dt.nanosecond() as i32,
            }
        }
    }

    #[cfg(all(feature = "serde", feature = "time"))]
    impl From<Timestamp> for String {
        fn from(value: Timestamp) -> Self {
            let odt = time::OffsetDateTime::try_from(value).expect("invalid date");
            odt.format(&time::format_description::well_known::Rfc3339)
                .expect("format is not rfc3339")
        }
    }

    #[cfg(feature = "time")]
    impl std::str::FromStr for Timestamp {
        type Err = time::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let timestamp =
                time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)?;

            Ok(Timestamp::from(timestamp))
        }
    }

    #[cfg(feature = "time")]
    impl TryFrom<String> for Timestamp {
        type Error = time::Error;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            <Timestamp as std::str::FromStr>::from_str(&value)
        }
    }

    #[cfg(feature = "time")]
    impl TryFrom<Timestamp> for time::OffsetDateTime {
        type Error = time::Error;

        fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
            let dt = time::OffsetDateTime::from_unix_timestamp(value.seconds)?;

            Ok(dt.replace_nanosecond(value.nanos as u32)?)
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
    use super::protobuf::DateItem;

    tonic::include_proto!("google.r#type");

    #[cfg(feature = "time")]
    impl From<time::OffsetDateTime> for Date {
        fn from(dt: time::OffsetDateTime) -> Self {
            Self {
                year: dt.year(),
                month: dt.month() as i32,
                day: dt.day() as i32,
            }
        }
    }

    #[cfg(feature = "time")]
    impl From<time::Date> for Date {
        fn from(value: time::Date) -> Self {
            Self {
                year: value.year(),
                month: value.month() as i32,
                day: value.day() as i32,
            }
        }
    }

    #[cfg(feature = "time")]
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

    #[cfg(feature = "time")]
    impl std::str::FromStr for Date {
        type Err = time::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let date =
                time::OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
                    .map(Date::from);

            match date {
                Ok(dt) => Ok(dt),
                Err(_e) => {
                    let my_format = time::macros::format_description!("[day]-[month]-[year]");
                    let date = time::Date::parse(&s, &my_format)?;
                    Ok(Date::from(date))
                }
            }
        }
    }

    #[cfg(feature = "time")]
    impl TryFrom<String> for Date {
        type Error = time::Error;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            <Date as std::str::FromStr>::from_str(&value)
        }
    }

    #[cfg(all(feature = "serde", feature = "time"))]
    impl TryFrom<DateItem> for Date {
        type Error = time::Error;

        fn try_from(value: DateItem) -> Result<Self, Self::Error> {
            match value {
                DateItem::String(ref string) => <Date as std::str::FromStr>::from_str(string),
                DateItem::Date { year, month, day } => Ok(Date { year, month, day }),
                DateItem::Timestamp { seconds, nanos } => {
                    let odt = time::OffsetDateTime::try_from(crate::google::protobuf::Timestamp {
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

    #[cfg(all(feature = "serde", feature = "time"))]
    impl From<Date> for String {
        fn from(value: Date) -> Self {
            format!("{}-{}-{}", value.year, value.month, value.day)
        }
    }
}

/// Custom types
#[cfg(feature = "configuration")]
pub mod custom {
    tonic::include_proto!("google.custom");
}
