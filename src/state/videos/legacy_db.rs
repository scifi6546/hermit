use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize)]
pub struct VideoData {
    pub star_rating: u32,    //star rating (eg 5 or 4 stars)
    pub rating: String,      //normal rating (eg pg, pg13)
    pub description: String, //Dexcription Of video
}
//Metadata struct is used to store data not directly related
//to the file such as thumbnail path
#[derive(Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub thumbnail_name: String,
    pub thumbnail_path: String,
    pub thumbnail_res: u32,
    pub video_data: VideoData,
    //to add stuff
}
#[derive(Clone, Serialize, Deserialize)]
pub struct FileData {
    pub file_name: String,
    pub name: String,
    pub file_path: String,
    pub extension: String,
    pub metadata: Metadata,
}
impl FileData {
    #[allow(dead_code)]
    pub fn is_video(&self) -> bool {
        if self.extension == "m4v".to_string()
            || self.extension == "ogg".to_string()
            || self.extension == "mp4".to_string()
        {
            return true;
        } else {
            return false;
        }
    }
}

//a playlist database
#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub video_paths: Vec<String>, //paths of all videos, path is a unique identifier
    pub name: String,             //name of playlist
}
#[derive(Clone, Serialize, Deserialize)]
pub struct FileDB {
    files: Vec<FileData>,
    db_path: String,
    file_path: String,
    version: u16,
    playlist: Vec<Playlist>,
}
impl FileDB {
    pub fn get_playlist_all(&self) -> Vec<Playlist> {
        return self.playlist.clone();
    }
    //makes a playlist
    //gets mutable iterator of FileData
    pub fn iter(&self) -> std::slice::Iter<'_, FileData> {
        let iterator = self.files.iter();
        return iterator;
    }
}
pub fn from_path(file_path: String) -> Result<FileDB, String> {
    let file_res = File::open(file_path);
    if file_res.is_ok() {
        let db_res = serde_json::from_reader(file_res.ok().unwrap());
        if db_res.is_ok() {
            return Ok(db_res.ok().unwrap());
        } else {
            return Err("failed to parse file".to_string());
        }
    } else {
        return Err("failed to find file".to_string());
    }
}
