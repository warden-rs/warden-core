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
}

impl Entity {
    fn package(&self) -> String {
        match self {
            Entity::Pacs002 | Entity::Pacs008| Entity::Payload => "iso20022",
            Entity::TypologyConfiguration | Entity::RoutingConfiguration => "configuration",
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
            Entity::RoutingConfiguration => "proto/configuration/routing.proto",
            Entity::TransactionRelationship => "proto/pseudonyms/transaction_relationship.proto",
            Entity::Account => "proto/pseudonyms/account.proto",
            Entity::Entity => "proto/pseudonyms/entity.proto",
            Entity::AccountHolder => "proto/pseudonyms/account_holder.proto",
            Entity::Payload => "proto/iso20022/payload.proto",
        }
        .into()
    }

    fn descriptor(&self) -> String {
        match self {
            Entity::Pacs008 => "pacs008",
            Entity::Pacs002 =>"pacs002",
            Entity::TypologyConfiguration => "typology_configuration",
            Entity::TransactionRelationship => "transaction_relationship",
            Entity::Account => "account",
            Entity::Entity => "entity",
            Entity::Payload => "payload",
            Entity::AccountHolder => "account_holder",
            Entity::RoutingConfiguration => "routing_configuration"
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
        protos.push(Entity::Payload);
    }

    if cfg!(feature = "configuration") {
        protos.push(Entity::TypologyConfiguration);
        protos.push(Entity::RoutingConfiguration);
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
            ".",
            "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"snake_case\")]",
        );

        #[cfg(feature = "openapi")]
        let config = config.type_attribute(".", "#[derive(utoipa::ToSchema)]");

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
