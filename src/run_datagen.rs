use std::{
    io,
    path::Path
};
use smol::{
    fs,
    process::{
        Command,
        Stdio
    }
};


pub async fn run_datagen<P>(output_dir : &P)
where
    P : AsRef<Path>
{
    let output_dir     = output_dir.as_ref();
    let output_tmp_dir = output_dir.with_added_extension("tmp");

    if (output_dir.is_dir() && (! output_tmp_dir.is_dir())) { return; }

    // Clear or create output dir.
    match (fs::remove_dir_all(&output_tmp_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::NotFound) => { },
        v => v.unwrap()
    };
    fs::create_dir_all(&output_tmp_dir).await.unwrap();

    println!("Running datagen...");
    Command::new("java")
        .arg("-DbundlerMainClass=net.minecraft.data.Main")
        .arg("-jar")
        .arg("../server.jar")
        .arg("--output")
        .arg("./")
        .arg("--server")
        .arg("--all")
        .arg("--dev")
        .current_dir(&output_tmp_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await.unwrap()
        .exit_ok().unwrap();

    match (fs::remove_dir_all(&output_dir).await) {
        Ok(_) => { },
        Err(err) if (err.kind() == io::ErrorKind::NotFound) => { },
        v => v.unwrap()
    };
    fs::rename(&output_tmp_dir, &output_dir).await.unwrap();
}
