#[cfg(any(
    feature = "iso20022",
    feature = "pseudonyms",
    feature = "configuration",
    feature = "payload"
))]
enum Entity {
    #[cfg(feature = "iso20022")]
    ISO2022,
    #[cfg(feature = "configuration")]
    Configuration,
    #[cfg(feature = "pseudonyms")]
    Pseudonyms,
    #[cfg(feature = "payload")]
    Payload,
}

#[cfg(any(
    feature = "iso20022",
    feature = "pseudonyms",
    feature = "configuration",
    feature = "payload"
))]
impl Entity {
    fn protos(&self) -> Vec<&'static str> {
        let mut res: Vec<&'static str> = vec![
            "proto/google/type/date.proto",
            "proto/google/type/money.proto",
            "proto/google/type/latlng.proto",
        ];

        #[cfg(any(feature = "iso20022", feature = "payload"))]
        fn iso20022_protos() -> Vec<&'static str> {
            vec![
                "proto/iso20022/pacs.008.001.12.proto",
                "proto/iso20022/pacs.002.001.12.proto",
            ]
        }

        #[cfg(any(feature = "configuration", feature = "payload"))]
        fn configuration_protos() -> Vec<&'static str> {
            vec![
                "proto/configuration/reload_event.proto",
                "proto/configuration/routing.proto",
                "proto/configuration/rule.proto",
                "proto/configuration/typology.proto",
            ]
        }

        match self {
            #[cfg(feature = "iso20022")]
            Entity::ISO2022 => {
                res.extend(iso20022_protos());
            }
            #[cfg(feature = "configuration")]
            Entity::Configuration => {
                res.extend(configuration_protos());
            }
            #[cfg(feature = "pseudonyms")]
            Entity::Pseudonyms => {
                res.extend(vec![
                    "proto/pseudonyms/account.proto",
                    "proto/pseudonyms/entity.proto",
                    "proto/pseudonyms/account_holder.proto",
                    "proto/pseudonyms/transaction_relationship.proto",
                ]);
            }
            #[cfg(feature = "payload")]
            Entity::Payload => {
                res.extend(iso20022_protos());
                res.extend(configuration_protos());
                res.push("proto/payload.proto");
            }
        }
        res
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");

    #[cfg(feature = "iso20022")]
    build_proto("iso20022", Entity::ISO2022);

    #[cfg(feature = "pseudonyms")]
    build_proto("pseudonyms", Entity::Pseudonyms);

    #[cfg(feature = "configuration")]
    build_proto("configuration", Entity::Configuration);

    #[cfg(feature = "payload")]
    build_proto("payload", Entity::Payload);

    Ok(())
}

#[cfg(any(
    feature = "iso20022",
    feature = "pseudonyms",
    feature = "configuration"
))]
fn build_proto(package: &str, entity: Entity) {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let config = tonic_build::configure().compile_well_known_types(true)
         .server_mod_attribute(
                &package,
                format!("#[cfg(feature = \"rpc-server-{package}\")] #[cfg_attr(docsrs, doc(cfg(feature = \"rpc-server-{package}\")))]"),
            )
            .client_mod_attribute(
                &package,
                format!("#[cfg(feature = \"rpc-client-{package}\")] #[cfg_attr(docsrs, doc(cfg(feature = \"rpc-client-{package}\")))]"),
            );

    #[cfg(feature = "configuration")]
    let config = configuration_extras(config);

    #[cfg(feature = "serde")]
    let config = add_serde(config);

    #[cfg(feature = "openapi")]
    let config = add_openapi(config);

    #[cfg(feature = "time")]
        let config = config.type_attribute(".google.protobuf.Timestamp", "#[serde(try_from = \"crate::google::helpers::time_util::DateItem\")] #[serde(into = \"String\")]")
        .type_attribute(".google.type.Date", "#[serde(try_from = \"crate::google::helpers::time_util::DateItem\")] #[serde(into = \"String\")]");

    let include_paths = &["proto"];

    config
        .file_descriptor_set_path(out_dir.join(format!("{package}_descriptor.bin")))
            .server_mod_attribute(
                &package,
                format!("#[cfg(feature = \"rpc-server-{package}\")] #[cfg_attr(docsrs, doc(cfg(feature = \"rpc-server-{package}\")))]"),
            )
            .client_mod_attribute(
                &package,
                format!("#[cfg(feature = \"rpc-client-{package}\")] #[cfg_attr(docsrs, doc(cfg(feature = \"rpc-client-{package}\")))]"),
            )
        .compile_well_known_types(true)
        .compile_protos(&entity.protos(), include_paths).unwrap();
}

#[cfg(feature = "configuration")]
fn configuration_extras(config: tonic_build::Builder) -> tonic_build::Builder {
    #[cfg(feature = "configuration")]
    config
        .type_attribute(
            ".configuration.typology.TypologyConfigurationRequest",
            "#[derive(Hash, Eq)]",
        )
        .type_attribute(
            ".configuration.rule.RuleConfigurationRequest",
            "#[derive(Hash, Eq)]",
        )
}

#[cfg(all(
    feature = "openapi",
    any(
        feature = "configuration",
        feature = "pseudonyms",
        feature = "iso20022"
    )
))]
fn add_openapi(config: tonic_build::Builder) -> tonic_build::Builder {
    config
        .type_attribute(".", "#[derive(utoipa::ToSchema)]")
        .type_attribute(
            ".configuration.typology.TypologyConfigurationRequest",
            "#[derive(utoipa::IntoParams)]",
        )
        .type_attribute(
            ".configuration.typology.DeleteTypologyConfigurationRequest",
            "#[derive(utoipa::IntoParams)]",
        )
        .type_attribute(
            ".configuration.rule.DeleteRuleConfigurationRequest",
            "#[derive(utoipa::IntoParams)]",
        )
        .type_attribute(
            ".configuration.rule.RuleConfigurationRequest",
            "#[derive(utoipa::IntoParams)]",
        )
        .field_attribute(
            "cre_dt_tm",
            "#[schema(value_type = String, format = DateTime)]",
        )
}

#[cfg(all(
    feature = "serde",
    any(
        feature = "configuration",
        feature = "pseudonyms",
        feature = "iso20022"
    )
))]
fn add_serde(config: tonic_build::Builder) -> tonic_build::Builder {
    let configuration = config.type_attribute(
        ".",
        "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"snake_case\")]",
    );

    #[cfg(feature = "configuration")]
    let configuration = update_serde_for_config(configuration);

    configuration
}

#[cfg(all(feature = "serde", feature = "configuration"))]
fn update_serde_for_config(config: tonic_build::Builder) -> tonic_build::Builder {
    config
            .field_attribute(".configuration.rule.Config.cases", "#[serde(default)]")
            .field_attribute(
                ".configuration.rule.Config.time_frames",
                "#[serde(default)]",
            )
            .field_attribute(
                ".configuration.typology.Expression.operator",
                "#[serde(with = \"operator_serde\")]",
            )
            .type_attribute(
                ".google.protobuf.Value",
                "#[serde(try_from = \"serde_json::Value\")] #[serde(into = \"crate::google::helpers::rule_help::GenericParameter\")]",
            )
}
