use pipeworkmc_data::{
    banner_pattern::BannerPattern,
    cat_variant::CatVariant,
    chicken_variant::ChickenVariant,
    cow_variant::CowVariant,
    damage_type::DamageType,
    dimension_type::DimensionType,
    frog_variant::FrogVariant,
    painting_variant::PaintingVariant,
    pig_variant::PigVariant,
    wolf_variant::WolfVariant,
    wolf_sound_variant::WolfSoundVariant,
    worldgen::biome::WorldgenBiome
};
use pipeworkmc_data::syndebug::{
    SynDebug,
    to_string as syndebug_to_string
};
use pipeworkmc_data::disqualified::ShortName;
use std::{
    fs::File,
    io::Write,
    path::Path
};
use serde::Deserialize as Deser;
use serde_json::from_reader as json_from_reader;
use smol::{
    fs,
    stream::StreamExt
};
use convert_case::{ Casing, Case };



pub async fn vanilla_datapack<P, Q>(cache_dir : P, generated_dir : Q)
where
    P : AsRef<Path>,
    Q : AsRef<Path>
{
    let cache_dir     = cache_dir.as_ref();
    let generated_dir = generated_dir.as_ref();

    println!("Generating banner patterns...");
    generate_from_dir::<BannerPattern>(
        &cache_dir.join("datagen/data/minecraft/banner_pattern"),
        &generated_dir.join("banner_pattern.rs"),
        "", "'static"
    ).await;

    println!("Generating cat variants...");
    generate_from_dir::<CatVariant>(
        &cache_dir.join("datagen/data/minecraft/cat_variant"),
        &generated_dir.join("cat_variant.rs"),
        "", ""
    ).await;

    println!("Generating chicken variants...");
    generate_from_dir::<ChickenVariant>(
        &cache_dir.join("datagen/data/minecraft/chicken_variant"),
        &generated_dir.join("chicken_variant.rs"),
        "", ""
    ).await;

    println!("Generating cow variants...");
    generate_from_dir::<CowVariant>(
        &cache_dir.join("datagen/data/minecraft/cow_variant"),
        &generated_dir.join("cow_variant.rs"),
        "", ""
    ).await;

    println!("Generating damage types...");
    generate_from_dir::<DamageType>(
        &cache_dir.join("datagen/data/minecraft/damage_type"),
        &generated_dir.join("damage_type.rs"),
        "", "'static"
    ).await;

    println!("Generating dimension types...");
    generate_from_dir::<DimensionType>(
        &cache_dir.join("datagen/data/minecraft/dimension_type"),
        &generated_dir.join("dimension_type.rs"),
        "", "'static"
    ).await;

    println!("Generating frog variants...");
    generate_from_dir::<FrogVariant>(
        &cache_dir.join("datagen/data/minecraft/frog_variant"),
        &generated_dir.join("frog_variant.rs"),
        "", ""
    ).await;

    println!("Generating painting variants...");
    generate_from_dir::<PaintingVariant>(
        &cache_dir.join("datagen/data/minecraft/painting_variant"),
        &generated_dir.join("painting_variant.rs"),
        "", ""
    ).await;

    println!("Generating pig variants...");
    generate_from_dir::<PigVariant>(
        &cache_dir.join("datagen/data/minecraft/pig_variant"),
        &generated_dir.join("pig_variant.rs"),
        "", ""
    ).await;

    println!("Generating wolf variants...");
    generate_from_dir::<WolfVariant>(
        &cache_dir.join("datagen/data/minecraft/wolf_variant"),
        &generated_dir.join("wolf_variant.rs"),
        "", "'static"
    ).await;

    println!("Generating wolf sound variants...");
    generate_from_dir::<WolfSoundVariant>(
        &cache_dir.join("datagen/data/minecraft/wolf_sound_variant"),
        &generated_dir.join("wolf_sound_variant.rs"),
        "", ""
    ).await;

    println!("Generating worldgen biomes...");
    generate_from_dir::<WorldgenBiome>(
        &cache_dir.join("datagen/data/minecraft/worldgen/biome"),
        &generated_dir.join("worldgen/biome.rs"),
        "", "'static"
    ).await;

}


async fn generate_from_dir<T>(source_dir : &Path, generated_path : &Path, impl_generics : &str, type_generics : &str)
where
    for<'l> T : Deser<'l> + SynDebug
{
    fs::create_dir_all(generated_path.parent().unwrap()).await.unwrap();
    let mut generated_file = File::create(generated_path).unwrap();

    let mut source_reader = fs::read_dir(source_dir).await.unwrap();
    let mut entries       = Vec::new();
    writeln!(generated_file, "impl<{impl_generics}> {}<{type_generics}> {{", ShortName::of::<T>()).unwrap();
    while let Some(entry) = source_reader.try_next().await.unwrap() {
        let path  = entry.path();
        let id    = path.with_extension("").file_name().unwrap().to_str().unwrap().to_string();
        println!("  {id}");
        let data  = json_from_reader::<_, T>(File::open(&path).unwrap()).unwrap();
        let ident = id.to_case(Case::Constant);
        writeln!(generated_file, "    /// `minecraft:{id}`").unwrap();
        write!(generated_file, "    pub const {ident} : Self = ").unwrap();
        write!(generated_file, "{}", syndebug_to_string(&data, true)).unwrap();
        writeln!(generated_file, ";").unwrap();
        entries.push((id, ident,));
    }

    writeln!(generated_file, "\n    /// All entries in the vanilla datapack.").unwrap();
    writeln!(generated_file, "    pub const VANILLA_ENTRIES : &'static [Self] = &[").unwrap();
    for (_, ident,) in &entries {
        writeln!(generated_file, "        Self::{ident},").unwrap();
    }
    writeln!(generated_file, "    ];\n").unwrap();

    writeln!(generated_file, "\n    /// All entries in the vanilla datapack, as a [`RegistryEntry`] slice.").unwrap();
    writeln!(generated_file, "\n    pub const VANILLA_REGISTRY_ENTRIES : &'static [RegistryEntry<Self>] = &[").unwrap();
    for (id, ident,) in &entries {
        writeln!(generated_file, "        RegistryEntry {{ id : Ident::new(\"minecraft:{}\"), data : Self::{ident} }},", id.escape_debug()).unwrap();
    }
    writeln!(generated_file, "    ];\n").unwrap();

    writeln!(generated_file, "}}").unwrap();
}
