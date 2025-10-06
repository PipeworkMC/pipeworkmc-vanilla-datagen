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


pub async fn packets<P, Q>(cache_dir : P, generated_dir : Q)
where
    P : AsRef<Path>,
    Q : AsRef<Path>
{
    let cache_dir      = cache_dir.as_ref();
    let generated_dir  = generated_dir.as_ref().join("packet");
    fs::create_dir_all(&generated_dir).await.unwrap();

    println!("Generating packets...");
    let data = json_from_reader::<_, PacketRegistry>(File::open(cache_dir.join("datagen/reports/packets.json")).unwrap()).unwrap();

    for (state, by_bound,) in [
        ("handshake", data.handshake,),
        ("status",    data.status,),
        ("login",     data.login,),
        ("config",    data.config,),
        ("play",      data.play,),
    ] {
        for (bound, by_id,) in [
            ("c2s", by_bound.c2s,),
            ("s2c", by_bound.s2c,)
        ] { if let Some(by_id) = by_id {
            println!("  {}{}", bound.to_uppercase(), state.to_case(Case::Pascal));
            let     generated_path = generated_dir.join(format!("{bound}_{state}.rs"));
            let mut generated_file = File::create(generated_path).unwrap();

            write!(generated_file, "macro packet_id {{\n").unwrap();
            for (name, PacketType { protocol_id },) in by_id {
                let name = name.split(":").skip(1).next().unwrap();
                write!(generated_file, "    ( {name:?} $(,)? ) => {{ 0x{protocol_id:0>2X} }},\n").unwrap();
            }
            write!(generated_file, "}}\n").unwrap();
        } }
    }

    // let mut generated_file = File::create(generated_path).unwrap();


}


#[derive(Deser)]
struct PacketRegistry {
    handshake : PacketRegistryByBound,
    status    : PacketRegistryByBound,
    login     : PacketRegistryByBound,
    #[serde(rename = "configuration")]
    config    : PacketRegistryByBound,
    play      : PacketRegistryByBound
}

#[derive(Deser)]
struct PacketRegistryByBound {
    #[serde(rename = "serverbound")]
    c2s : Option<HashMap<String, PacketType>>,
    #[serde(rename = "clientbound")]
    s2c : Option<HashMap<String, PacketType>>
}

#[derive(Deser)]
struct PacketType {
    protocol_id : u8
}
