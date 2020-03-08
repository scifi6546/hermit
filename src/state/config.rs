use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
const CONFIG_VERSION: u32 = 1;
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub passwd: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct VideoConf {
    pub video_path: String,
    pub thumbnails: String,
    pub playlists: Vec<u8>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub users: Vec<User>,
    pub videos: VideoConf,
    pub thumb_res: u32,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    pub version: u32, //version number of config file
    pub data: Config, //data for configuration
}
pub fn empty() -> Config {
    return Config {
        users: [].to_vec(),
        videos: VideoConf {
            video_path: "".to_string(),
            thumbnails: "thumbnails".to_string(),
            playlists: [].to_vec(),
        },
        thumb_res: 0,
    };
}
fn get_config() -> std::result::Result<ConfigFile, String> {
    println!("ran?");

    let file = File::open("config.json");
    if file.is_ok() {
        let mut string = String::new();
        let res = file.unwrap().read_to_string(&mut string);
        let config = serde_json::from_str::<ConfigFile>(&string);
        if config.is_ok() && res.is_ok() {
            return Ok(config.unwrap());
        }
        return Err("config file not parsed".to_string());
    }
    return Err("config file not found".to_string());
}
fn print_config(input: Config) {
    println!("Users: ");
    for user in input.users {
        println!("   username: {}", user.username);
        println!("   password: {}", user.passwd);
    }
    println!("Video: ");
    println!("  video_path: {}", input.videos.video_path);
    println!("  thumbnail_path: {}", input.videos.thumbnails);
}
pub fn load_config() -> Result<Config, String> {
    let result = get_config();
    if result.is_ok() {
        let config_out = result.unwrap();
        if config_out.version < CONFIG_VERSION {
            let config = convert_config(config_out);
            if config.is_ok() {
                return Ok(config.unwrap().data);
            } else {
                return Err("config not converted properly".to_string());
            }
        }
        print_config(config_out.clone().data);
        info!("loaded config file successfully");

        return Ok(config_out.data);
    }
    return Err(result.err().unwrap());
}
pub fn convert_config(config_in: ConfigFile) -> Result<ConfigFile, String> {
    //there is only 1 version so no converting needed yet
    return Ok(config_in);
}
pub fn write_conf(input: Config) -> std::io::Result<()> {
    let mut file = File::create("config.json")?;

    let write_string = serde_json::to_string(&ConfigFile {
        version: CONFIG_VERSION,
        data: input,
    })
    .unwrap();
    debug!("writing config string: {}", write_string);
    return file.write_all(&write_string.into_bytes());
}
