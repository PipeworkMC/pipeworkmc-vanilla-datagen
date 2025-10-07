use serde::Deserialize as Deser;
use std::path::Path;
use smol::{
    fs::{ self, File },
    io
};


pub async fn download_server_files<P, Q>(
    version              : &str,
    server_jar_path      : P,
    server_mappings_path : Q
)
where
    P : AsRef<Path>,
    Q : AsRef<Path>
{
    let server_jar_path          = server_jar_path.as_ref();
    let server_mappings_path     = server_mappings_path.as_ref();
    let server_jar_tmp_path      = server_jar_path.with_added_extension("tmp");
    let server_mappings_tmp_path = server_mappings_path.with_added_extension("tmp");

    let download_server_jar      = (! server_jar_path      .is_file()) || (server_jar_tmp_path      .is_file());
    let download_server_mappings = (! server_mappings_path .is_file()) || (server_mappings_tmp_path .is_file());

    if (! (download_server_jar || download_server_mappings)) { return; }

    println!("Fetching version manifest...");
    let version_manifest = surf::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .send()
        .await.unwrap()
        .body_json::<VersionManifest>()
        .await.unwrap();
    let version_url = version_manifest.versions
        .into_iter()
        .find(|v| v.id == version)
        .unwrap()
        .url;
    println!("Fetching {version} version info...");
    let version_info = surf::get(version_url)
        .send()
        .await.unwrap()
        .body_json::<VersionInfo>()
        .await.unwrap();

    if (download_server_jar) {
        println!("Downloading server jar...");

        let mut server_jar_file = File::create(&server_jar_tmp_path).await.unwrap();
        io::copy(
            surf::get(&version_info.downloads.server.url)
                .send()
                .await.unwrap(),
            &mut server_jar_file
        ).await.unwrap();

        match (fs::remove_dir_all(&server_jar_path).await) {
            Ok(_) => { },
            Err(err) if (err.kind() == io::ErrorKind::NotFound) => { },
            v => v.unwrap()
        };
        fs::rename(&server_jar_tmp_path, &server_jar_path).await.unwrap();
    }

    if (! server_mappings_path.is_file()) {
        println!("Downloading server mappings...");

        let mut server_mappings_file = File::create(&server_mappings_tmp_path).await.unwrap();
        io::copy(
            surf::get(&version_info.downloads.server_mappings.url)
                .send()
                .await.unwrap(),
            &mut server_mappings_file
        ).await.unwrap();

        match (fs::remove_dir_all(&server_mappings_path).await) {
            Ok(_) => { },
            Err(err) if (err.kind() == io::ErrorKind::NotFound) => { },
            v => v.unwrap()
        };
        fs::rename(&server_mappings_tmp_path, &server_mappings_path).await.unwrap();
    }
}


#[derive(Deser)]
struct VersionManifest {
    versions : Vec<ManifestVersion>
}
#[derive(Deser)]
struct ManifestVersion {
    id  : String,
    url : String
}
#[derive(Deser)]
struct VersionInfo {
    downloads : VersionDownloads
}
#[derive(Deser)]
struct VersionDownloads {
    server          : VersionDownload,
    server_mappings : VersionDownload
}
#[derive(Deser)]
struct VersionDownload {
    url : String
}
