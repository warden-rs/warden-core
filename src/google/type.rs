include!(concat!(env!("OUT_DIR"), "/google.r#type.rs"));

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
        let date = time::OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
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

#[cfg(feature = "time")]
impl TryFrom<String> for Date {
    type Error = time::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        <Date as std::str::FromStr>::from_str(&value)
    }
}

#[cfg(feature = "time")]
impl TryFrom<super::protobuf::DateItem> for Date {
    type Error = time::Error;

    fn try_from(value: super::protobuf::DateItem) -> Result<Self, Self::Error> {
        match value {
            super::protobuf::DateItem::String(ref string) => {
                <Date as std::str::FromStr>::from_str(string)
            }
            #[cfg(feature = "iso20022")]
            super::protobuf::DateItem::Date { year, month, day } => Ok(Date { year, month, day }),
            super::protobuf::DateItem::Timestamp { seconds, nanos } => {
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
