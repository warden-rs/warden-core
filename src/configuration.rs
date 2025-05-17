tonic::include_proto!("configuration");
/// Configuration file descriptor
pub const CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("configuration_descriptor");

pub mod typology {
    tonic::include_proto!("configuration.typology");

    /// Typology Configuration file descriptor
    pub const TYPOLOGY_CONFIGURATION_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("typology_configuration_descriptor");

    #[cfg(feature = "serde")]
    mod operator_serde {
        use super::Operator;
        use serde::{self, Deserialize, Deserializer, Serializer};

        pub fn serialize<S>(operator: &i32, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let operator = Operator::try_from(*operator);
            if let Ok(d) = operator {
                return s.serialize_str(d.as_str_name());
            }
            s.serialize_none()
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;

            if let Some(s) = s {
                let op = Operator::from_str_name(&s)
                    .ok_or_else(|| serde::de::Error::custom("unsupported"))?
                    as i32;
                return Ok(op);
            }

            Err(serde::de::Error::custom("deserialise error for operator"))
        }
    }

    impl From<Operator> for String {
        fn from(value: Operator) -> Self {
            value.as_str_name().to_owned()
        }
    }

    impl TryFrom<String> for Operator {
        type Error = String;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            let value = value.to_uppercase();
            Operator::from_str_name(&value)
                .ok_or_else(|| format!("unsupported operator: {}", value))
        }
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
