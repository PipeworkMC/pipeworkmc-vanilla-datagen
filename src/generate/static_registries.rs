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

    write!(generated_file, "/// A character type.\n").unwrap();
    write!(generated_file, "#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]\n").unwrap();
    write!(generated_file, "pub enum CharacterType {{\n").unwrap();
    for (id, _,) in &entity_types.entries {
        println!("  {}", id.path());
        let ident = id.path().to_case(Case::Pascal);
        write!(generated_file, "    /// `minecraft:{id}`\n").unwrap();
        write!(generated_file, "    {ident},\n").unwrap();
    }
    write!(generated_file, "}}\n").unwrap();
    write!(generated_file, "impl CharacterType {{\n").unwrap();
    write!(generated_file, "    /// Returns this [`CharacterType`]'s protocol ID.\n").unwrap();
    write!(generated_file, "    pub const fn protocol_id(&self) -> u32 {{\n").unwrap();
    write!(generated_file, "        match (self) {{\n").unwrap();
    for (id, StaticRegistryProtocolId { protocol_id },) in &entity_types.entries {
        let ident = id.path().to_case(Case::Pascal);
        write!(generated_file, "            Self::{ident} => {protocol_id},\n").unwrap();
    }
    write!(generated_file, "        }}\n").unwrap();
    write!(generated_file, "    }}\n").unwrap();
    write!(generated_file, "}}\n").unwrap();
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
