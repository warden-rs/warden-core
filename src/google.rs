/// Well known types
pub mod protobuf {

    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));

    #[cfg(feature = "time")]
    impl From<time::OffsetDateTime> for Timestamp {
        fn from(dt: time::OffsetDateTime) -> Self {
            Timestamp {
                seconds: dt.unix_timestamp(),
                nanos: dt.nanosecond() as i32,
            }
        }
    }

    #[cfg(feature = "time")]
    impl std::str::FromStr for Timestamp {
        type Err = void::Void;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let time =
                time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
                    .unwrap();
            Ok(Timestamp::try_from(time).unwrap())
        }
    }

    #[cfg(feature = "time")]
    impl TryFrom<Timestamp> for time::OffsetDateTime {
        type Error = time::error::ComponentRange;

        fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
            let dt = time::OffsetDateTime::from_unix_timestamp(value.seconds)?;

            dt.replace_nanosecond(value.nanos as u32)
        }
    }

    #[cfg(feature = "time")]
    pub(crate) mod serialise_dt {
        use std::{fmt, marker::PhantomData, str::FromStr};

        use serde::{
            Deserialize, Deserializer, Serializer,
            de::{self, MapAccess, Visitor},
        };
        use time::OffsetDateTime;

        use super::Timestamp;

        pub fn serialize<S: Serializer>(
            datetime: &Timestamp,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            let value = OffsetDateTime::try_from(*datetime).unwrap();
            time::serde::rfc3339::serialize(&value, serializer)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
        where
            D: Deserializer<'de>,
        {
            // This is a Visitor that forwards string types to T's `FromStr` impl and
            // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
            // keep the compiler from complaining about T being an unused generic type
            // parameter. We need T in order to know the Value type for the Visitor
            // impl.
            struct StringOrStruct(PhantomData<fn() -> Timestamp>);

            impl<'de> Visitor<'de> for StringOrStruct {
                type Value = Timestamp;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("string or map")
                }

                fn visit_str<E>(self, value: &str) -> Result<Timestamp, E>
                where
                    E: de::Error,
                {
                    Ok(FromStr::from_str(value).unwrap())
                }

                fn visit_map<M>(self, map: M) -> Result<Timestamp, M::Error>
                where
                    M: MapAccess<'de>,
                {
                    let ds = de::value::MapAccessDeserializer::new(map);
                    let value = Timestamp::deserialize(ds)?;
                    Ok(Timestamp {
                        seconds: value.seconds,
                        nanos: value.nanos,
                    })
                    // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
                    // into a `Deserializer`, allowing it to be used as the input to T's
                    // `Deserialize` implementation. T then deserializes itself using
                }
            }

            deserializer.deserialize_any(StringOrStruct(PhantomData))
        }

        pub mod option {

            use super::*;

            pub fn serialize<S: Serializer>(
                option: &Option<Timestamp>,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                match option {
                    Some(ts) => {
                        let dt = OffsetDateTime::try_from(ts.clone())
                            .map_err(serde::ser::Error::custom)?;
                        serializer.serialize_some(&dt)
                    }
                    None => serializer.serialize_none(),
                }
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Option<Timestamp>, D::Error> {
                println!("DEBUG: option::deserialize hit");
                struct StringOrStructOpt;

                impl<'de> Visitor<'de> for StringOrStructOpt {
                    type Value = Option<Timestamp>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("an optional RFC3339 string or a Timestamp map")
                    }

                    fn visit_none<E>(self) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok(None)
                    }

                    fn visit_unit<E>(self) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok(None)
                    }

                    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        super::deserialize(deserializer).map(Some)
                    }
                }

                deserializer.deserialize_option(StringOrStructOpt)
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
    impl From<Date> for time::Date {
        fn from(value: Date) -> Self {
            Self::from_calendar_date(
                value.year,
                time::Month::try_from(value.month as u8).expect("invalid month"),
                value.day as u8,
            )
            .expect("invalid date")
        }
    }
}

/// Custom types
#[cfg(feature = "configuration")]
pub mod custom {
    tonic::include_proto!("google.custom");
}
