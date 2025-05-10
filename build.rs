enum Entity {
    Pacs008,
    Pacs002,
    Payload,
    TypologyConfiguration,
    TransactionRelationship,
    Account,
    Entity,
    AccountHolder,
    RoutingConfiguration,
    RuleConfiguration,
    ReloadEvent,
    MathJSON,
}

impl Entity {
    fn package(&self) -> String {
        match self {
            Entity::Pacs002 | Entity::Pacs008 | Entity::Payload => "iso20022",
            Entity::TypologyConfiguration
            | Entity::RoutingConfiguration
            | Entity::RuleConfiguration
            | Entity::ReloadEvent
            | Entity::MathJSON => "configuration",
            Entity::TransactionRelationship
            | Entity::Entity
            | Entity::Account
            | Entity::AccountHolder => "pseudonyms",
        }
        .into()
    }
    fn path(&self) -> String {
        match self {
            Entity::Pacs008 => "proto/iso20022/pacs.008.001.12.proto",
            Entity::Pacs002 => "proto/iso20022/pacs.002.001.12.proto",
            Entity::TypologyConfiguration => "proto/configuration/typology.proto",
            Entity::MathJSON => "proto/configuration/typology/mathjson.proto",
            Entity::RoutingConfiguration => "proto/configuration/routing.proto",
            Entity::RuleConfiguration => "proto/configuration/rule.proto",
            Entity::TransactionRelationship => "proto/pseudonyms/transaction_relationship.proto",
            Entity::Account => "proto/pseudonyms/account.proto",
            Entity::Entity => "proto/pseudonyms/entity.proto",
            Entity::AccountHolder => "proto/pseudonyms/account_holder.proto",
            Entity::Payload => "proto/payload.proto",
            Entity::ReloadEvent => "proto/configuration/reload_event.proto",
        }
        .into()
    }

    fn descriptor(&self) -> String {
        match self {
            Entity::Pacs008 => "pacs008",
            Entity::Pacs002 => "pacs002",
            Entity::TypologyConfiguration => "typology_configuration",
            Entity::RuleConfiguration => "rule_configuration",
            Entity::TransactionRelationship => "transaction_relationship",
            Entity::MathJSON => "mathjson",
            Entity::Account => "account",
            Entity::Entity => "entity",
            Entity::Payload => "payload",
            Entity::AccountHolder => "account_holder",
            Entity::RoutingConfiguration => "routing_configuration",
            Entity::ReloadEvent => "configuration",
        }
        .into()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");

    let mut protos = vec![];
    if cfg!(feature = "iso20022") {
        protos.push(Entity::Pacs008);
        protos.push(Entity::Pacs002);
    }

    if cfg!(feature = "payload") {
        protos.push(Entity::Payload);
    }

    if cfg!(feature = "configuration") {
        protos.push(Entity::RuleConfiguration);
        protos.push(Entity::TypologyConfiguration);
        protos.push(Entity::RoutingConfiguration);
        protos.push(Entity::ReloadEvent);
        protos.push(Entity::MathJSON);
    }

    if cfg!(feature = "pseudonyms") {
        protos.push(Entity::AccountHolder);
        protos.push(Entity::Account);
        protos.push(Entity::Entity);
        protos.push(Entity::TransactionRelationship);
    }

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);

    for proto in protos {
        let path = proto.path();
        let package = proto.package();
        let descriptor = proto.descriptor();

        let config = tonic_build::configure();

        #[cfg(feature = "serde")]
        let config = config.type_attribute(
            ".configuration.typology.TypologyConfigurationRequest",
            "#[derive(Hash, Eq)]",
        );

        #[cfg(feature = "serde")]
        let config = config.type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"snake_case\")]",
        );

        #[cfg(feature = "serde-time")]
        let config = config
            .type_attribute(
                ".google.protobuf.Timestamp",
                "#[serde(try_from = \"String\")]",
            )
            .type_attribute(".google.type.Date", "#[serde(try_from = \"String\")]");

        #[cfg(feature = "openapi")]
        let config = config
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
            );

        #[cfg(all(feature = "openapi", feature = "configuration"))]
        let config = config.field_attribute(
            ".configuration.typology.mathjson.MathJson",
            "#[schema(no_recursion)]",
        );

        #[cfg(all(feature = "serde", feature = "configuration"))]
        let config = config
            .field_attribute(".configuration.rule.Config.cases", "#[serde(default)]")
            .field_attribute(".configuration.rule.Config.time_frames", "#[serde(default)]");

        config
        .file_descriptor_set_path(out_dir.join(format!("{descriptor}_descriptor.bin")))
            .server_mod_attribute(
                &package,
                format!("#[cfg(feature = \"rpc-server-{package}\")] #[cfg_attr(docsrs, doc(cfg(feature = \"rpc-server-{package}\")))]"),
            )
            .client_mod_attribute(
                &package,
                format!("#[cfg(feature = \"rpc-client-{package}\")] #[cfg_attr(docsrs, doc(cfg(feature = \"rpc-client-{package}\")))]"),
            )
        .compile_well_known_types(true)
        .compile_protos(&[path], &[""])?;
    }

    Ok(())
}
