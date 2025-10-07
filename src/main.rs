#![feature(
    exit_status_error,
    path_add_extension
)]


use std::{
    fs::File,
    io::{ self, Write },
    path::Path
};
use smol::fs;


mod download_server_files;
use download_server_files::download_server_files;

mod run_datagen;
use run_datagen::run_datagen;

mod generate;


fn main() { smol::block_on(async {
    let version       = (772, "1.21.8",);
    let generated_dir = AsRef::<Path>::as_ref("output/generated");
    let cache_dir     = format!("output/cache/{}", version.1);
    let cache_dir     = AsRef::<Path>::as_ref(&cache_dir);

    let server_jar_path      = cache_dir.join("server.jar");
    let server_mappings_path = cache_dir.join("mappings.txt");
    let datagen_path         = cache_dir.join("datagen");

    // Clear or create generated dir.
    match (fs::remove_dir_all(generated_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::NotFound) => { },
        v => v.unwrap()
    };

    // Create cache dir.
    match (fs::create_dir_all(&cache_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::AlreadyExists) => { },
        v => v.unwrap()
    }

    // Fetch files and generate output files.
    download_server_files(version.1, &server_jar_path, &server_mappings_path).await;
    run_datagen(&datagen_path).await;
    generate::packets(&cache_dir, &generated_dir).await;
    generate::static_registries(&cache_dir, &generated_dir).await;
    generate::vanilla_datapack(&cache_dir, &generated_dir).await;

    {
        let     generated_path = generated_dir.join("version.rs");
        let mut generated_file = File::create(generated_path).unwrap();
        writeln!(generated_file, "impl Version {{").unwrap();
        writeln!(generated_file, "    /// The current version supported by pipework.").unwrap();
        writeln!(generated_file, "    pub const CURRENT : Self = Self::by_id({}).unwrap();", version.0).unwrap();
        writeln!(generated_file, "}}").unwrap();
    }

}) }
