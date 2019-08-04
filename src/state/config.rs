use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read,Write};
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct User{
    pub username: String,
    pub passwd: String
}
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct VideoConf{
    pub video_path: String,
    pub thumbnails: String,
    pub playlists: Vec<u8>
}
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Config{
    pub users:Vec<User>,
    pub videos:VideoConf

}
pub fn empty()->Config{
    return Config{users:[].to_vec(),
    videos:VideoConf{
        video_path: "".to_string(),
        thumbnails: "thumbnails".to_string(),
        playlists: [].to_vec(),
    },
    }
}
fn get_config()->std::result::Result<Config,String>{
    println!("ran?");
    
    let file=File::open("config.json");
    if file.is_ok(){
        let mut string = String::new();
        let res = file.unwrap().read_to_string(&mut string);
        let config = serde_json::from_str(&string);
        if config.is_ok() && res.is_ok(){
            return Ok(config.unwrap());
        }
        return Err("config file not parsed".to_string());
    }
    return Err("config file not found".to_string());
}
fn print_config(input: Config){
    println!("Users: ");
    for user in input.users{
        println!("   username: {}",user.username);
        println!("   password: {}",user.passwd);
    }
    println!("Video: ");
    println!("  video_path: {}",input.videos.video_path);
    println!("  thumbnail_path: {}",input.videos.thumbnails);
}
pub fn load_config()->Result<Config,String>{
    let result=get_config();
    if result.is_ok(){
        let config_out=result.unwrap();
        print_config(config_out.clone());
        return Ok(config_out)
    }
    return Err(result.err().unwrap());
}
pub fn write_conf(input: Config)->std::io::Result<()>{
    let mut file = File::create("config.json")?;
    let write_string = serde_json::to_string(&input).unwrap();
    println!("write_string: {}",write_string);
    return file.write_all(&write_string.into_bytes());
}
