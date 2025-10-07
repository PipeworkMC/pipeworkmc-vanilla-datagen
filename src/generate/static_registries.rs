use pipeworkmc_data::ident::Ident;
use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::Path
};
use smol::fs;
use serde::Deserialize as Deser;
use serde_json::from_reader as json_from_reader;
use convert_case::{ Casing, Case };


pub async fn static_registries<P, Q>(cache_dir : P, generated_dir : Q)
where
    P : AsRef<Path>,
    Q : AsRef<Path>
{
    let cache_dir     = cache_dir.as_ref();
    let generated_dir = generated_dir.as_ref();

    let data = json_from_reader::<_, StaticRegistries>(File::open(cache_dir.join("datagen/reports/registries.json")).unwrap()).unwrap();

    println!("Generating entity types...");
    entity_types(&generated_dir.join("entity_type.rs"), data.entity_type).await;
}
#[derive(Deser)]
struct StaticRegistries {
    #[serde(rename = "minecraft:entity_type")]
    entity_type : EntityTypeStaticRegistry
}


async fn entity_types(generated_path : &Path, entity_types : EntityTypeStaticRegistry) {
    fs::create_dir_all(generated_path.parent().unwrap()).await.unwrap();
    let mut generated_file = File::create(generated_path).unwrap();

    writeln!(generated_file, "/// A character type.").unwrap();
    writeln!(generated_file, "#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]").unwrap();
    writeln!(generated_file, "pub enum CharacterType {{").unwrap();
    for id in entity_types.entries.keys() {
        println!("  {}", id.path());
        let ident = id.path().to_case(Case::Pascal);
        writeln!(generated_file, "    /// `minecraft:{id}`").unwrap();
        writeln!(generated_file, "    {ident},").unwrap();
    }
    writeln!(generated_file, "}}").unwrap();
    writeln!(generated_file, "impl CharacterType {{").unwrap();
    writeln!(generated_file, "    /// Returns this [`CharacterType`]'s protocol ID.").unwrap();
    writeln!(generated_file, "    pub const fn protocol_id(&self) -> u32 {{").unwrap();
    writeln!(generated_file, "        match (self) {{").unwrap();
    for (id, StaticRegistryProtocolId { protocol_id },) in &entity_types.entries {
        let ident = id.path().to_case(Case::Pascal);
        writeln!(generated_file, "            Self::{ident} => {protocol_id},").unwrap();
    }
    writeln!(generated_file, "        }}").unwrap();
    writeln!(generated_file, "    }}").unwrap();
    writeln!(generated_file, "}}").unwrap();
}
#[derive(Deser)]
struct EntityTypeStaticRegistry {
    entries : HashMap<Ident, StaticRegistryProtocolId>
}

#[derive(Deser)]
#[serde(deny_unknown_fields)]
struct StaticRegistryProtocolId {
    protocol_id : u32
}
