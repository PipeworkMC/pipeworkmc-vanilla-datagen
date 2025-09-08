#![feature(
    exit_status_error,
    path_add_extension
)]


use std::io;
use smol::fs;


mod download_server_files;
use download_server_files::download_server_files;

mod run_datagen;
use run_datagen::run_datagen;


fn main() { smol::block_on(async {
    let version       = "1.21.8";
    let generated_dir = "output/generated";
    let cache_dir     = format!("output/cache/{version}");

    let server_jar_path      = format!("{cache_dir}/server.jar");
    let server_mappings_path = format!("{cache_dir}/mappings.txt");
    let datagen_path         = format!("{cache_dir}/datagen");

    // Clear or create generated dir.
    match (fs::remove_dir_all(generated_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::NotFound) => { },
        v => v.unwrap()
    };
    fs::create_dir_all(generated_dir).await.unwrap();

    // Create cache dir.
    match (fs::create_dir_all(cache_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::AlreadyExists) => { },
        v => v.unwrap()
    }

    download_server_files(version, &server_jar_path, &server_mappings_path).await;

    run_datagen(&datagen_path).await;

}) }
