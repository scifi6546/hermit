use serde::{Deserialize, Serialize};
use std::path::Path;
mod legacy_db;
use gulkana;
use std::fs;
mod thumbnail;
use gulkana::ServiceClient;
#[derive(Clone, Serialize, Deserialize, Debug, std::cmp::PartialEq)]
pub enum FileTypes {
    Video,
    GbaRom,
    GBRom,
    Unknown,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VideoData {
    pub star_rating: u32,    //star rating (eg 5 or 4 stars)
    pub rating: String,      //normal rating (eg pg, pg13)
    pub description: String, //Dexcription Of video
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub thumbnail_name: String,
    pub thumbnail_path: String,
    pub thumbnail_res: u32,
    pub video_data: VideoData,
    //to add stuff
}
fn new_metadata() -> Metadata {
    return Metadata {
        thumbnail_path: "".to_string(),
        thumbnail_name: "".to_string(),

        thumbnail_res: 0,
        video_data: VideoData {
            star_rating: 0,
            rating: "".to_string(),
            description: "".to_string(),
        },
    };
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileData {
    pub file_name: String,
    pub name: String,
    pub file_path: String,
    pub extension: String,
    pub metadata: Metadata,
    pub file_type: FileTypes,
}
impl FileData {
    pub fn gen_file_type(&mut self) -> FileTypes {
        self.file_type = match self.extension.as_str() {
            "m4v" => FileTypes::Video,
            "ogg" => FileTypes::Video,
            "mp4" => FileTypes::Video,
            "gba" => FileTypes::GbaRom,
            "gb" => FileTypes::GBRom,
            _ => FileTypes::Unknown,
        };
        return self.file_type.clone();
    }
    pub fn can_show_file(&self) -> bool {
        info!(
            "file_type: {:?} extension: {}",
            self.file_type, self.extension
        );
        match self.file_type {
            FileTypes::Video => true,
            FileTypes::GBRom => true,
            _ => false,
        }
    }
}
impl From<legacy_db::FileData> for FileData {
    fn from(f_in: legacy_db::FileData) -> FileData {
        let mut file = FileData {
            file_name: f_in.file_name,
            name: f_in.name,
            file_path: f_in.file_path,
            extension: f_in.extension,
            file_type: FileTypes::Video,
            metadata: Metadata {
                thumbnail_name: f_in.metadata.thumbnail_name,
                thumbnail_path: f_in.metadata.thumbnail_path,
                thumbnail_res: f_in.metadata.thumbnail_res,
                video_data: VideoData {
                    star_rating: f_in.metadata.video_data.star_rating,
                    rating: f_in.metadata.video_data.rating,
                    description: f_in.metadata.video_data.description,
                },
            },
        };
        file.gen_file_type();
        return file;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VideoHtml {
    pub name: String,
    pub url: String,
    pub thumbnail_url: String,
    pub html_url: String,
    pub path: String,
    pub video_data: VideoData,
}
//used to edit video
#[derive(Clone, Serialize, Deserialize)]
pub struct VideoEditData {
    pub star_rating: u32,    //star rating (eg 5 or 4 stars)
    pub rating: String,      //normal rating (eg pg, pg13)
    pub description: String, //Dexcription Of video
    pub name: String,        //name to change to
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistMeta {}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DirectoryTypes {
    Directory,
    Playlist(PlaylistMeta),
}
pub struct VideoDB {
    database: ServiceClient<String, FileData, DirectoryTypes>,
    database_path: Option<String>,
    thumb_dir: String,
    thumb_res: u32,
    source_dir: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct HtmlPlaylist {
    pub videos: Vec<VideoHtml>, //paths of all videos, path is a unique identifier
    pub name: String,           //name of playlist
}
impl VideoDB {
    fn make_thumbnails(&mut self) -> Result<String, String> {
        let mut keys = vec![];
        for (key, _file) in self.database.iter_data().unwrap() {
            keys.push(key.clone());
        }
        for key in keys {
            //make thumbnail
            let file_res = self.database.get(key);
            if file_res.is_ok() {
                let mut file = file_res.ok().unwrap().clone();
                if file.file_type == FileTypes::Video {
                    let thumb_res = thumbnail::make_thumb(
                        file.file_path.clone(),
                        self.thumb_dir.clone(),
                        self.thumb_res.clone(),
                    );
                    if thumb_res.is_ok() {
                        let thumb = thumb_res.unwrap();
                        file.metadata = Metadata {
                            thumbnail_path: thumb.path,
                            thumbnail_name: thumb.name,
                            thumbnail_res: thumb.resolution,
                            video_data: file.metadata.video_data.clone(),
                        };
                        let res = self.database.set_data(key, file);
                        if res.is_err() {
                            return Err("failed to set data".to_string());
                        }
                    } else {
                        return Err(thumb_res.err().unwrap());
                    }
                }
            }
        }
        return Ok("sucessfully made thumbnails".to_string());
    }
    pub fn add_video(
        &mut self,
        file_name: String,
        mut video_data: FileData,
    ) -> Result<String, String> {
        video_data.gen_file_type();

        let res = self.database.set_data(file_name, video_data);
        if res.is_ok() {
            return Ok("".to_string());
        } else {
            return Err("add failed".to_string());
        }
    }
    pub fn get_vid_html_vec(
        &self,
        path_base: String,
        html_path_base: String,
        thumbnail_base: String,
    ) -> Vec<VideoHtml> {
        let mut vec_out: Vec<VideoHtml> = Vec::new();
        debug!("database size when getting data: {}", self.database.len().ok().unwrap());
        for (_key, file) in self.database.iter_data().unwrap(){
            if file.can_show_file() {
                let name = file.name.clone();
                let mut file_url = path_base.clone();
                file_url.push_str(&name);
                let mut html_url = html_path_base.clone();
                html_url.push_str(&name);

                let video_data = VideoData {
                    rating: file.metadata.video_data.rating.clone(),
                    star_rating: file.metadata.video_data.star_rating,
                    description: file.metadata.video_data.description.clone(),
                };
                let mut thumbnail_name = thumbnail_base.clone();
                thumbnail_name.push_str(&file.metadata.thumbnail_name.clone());
                vec_out.push(VideoHtml {
                    name: file.name.clone(),
                    url: file_url.clone(),
                    thumbnail_url: thumbnail_name,
                    html_url: html_url.clone(),
                    path: file.file_path.clone(),
                    video_data: video_data,
                });
            }
        }
        return vec_out;
    }
    pub fn get_vid_html_from_path(
        &self,
        path_base: String,
        thumbnail_base: String,
        vid_path: String,
    ) -> Result<VideoHtml, String> {
        let res = self.database.get(vid_path);
        if res.is_ok() {
            let file = res.ok().unwrap();
            let mut thumbnail_name = thumbnail_base.clone();
            let mut url = path_base.clone();
            thumbnail_name.push_str(&file.metadata.thumbnail_name);
            url.push_str(&file.name);
            let video_data = VideoData {
                rating: file.metadata.video_data.rating.clone(),
                star_rating: file.metadata.video_data.star_rating,
                description: file.metadata.video_data.description.clone(),
            };
            return Ok(VideoHtml {
                name: file.name.clone(),
                url: url.clone(),
                thumbnail_url: thumbnail_name,
                html_url: url,
                path: file.file_path.clone(),
                video_data: video_data,
            });
        } else {
            return Err("Key not found".to_string());
        }
    }
    pub fn edit_video_data_path(
        &mut self,
        path: String,
        to_change_to: VideoEditData,
    ) -> Result<String, String> {
        let res = self.database.get(path);
        if res.is_ok() {
            let mut data = res.ok().unwrap().clone();
            data.name = to_change_to.name;
            data.metadata.video_data = VideoData {
                rating: to_change_to.rating,
                star_rating: to_change_to.star_rating,
                description: to_change_to.description,
            };
            let res = self.database.set_data(path, data);
            if res.is_ok() {
                return Ok("success".to_string());
            } else {
                return Err("Failed to Update Video".to_string());
            }
        } else {
            return Err("file not found".to_string());
        }
    }
    pub fn add_playlist(
        &mut self,
        playlist_name: String,
        video_paths: Vec<String>,
    ) -> Result<String, String> {
        let res = self.database.overwrite_link(
            playlist_name,
            video_paths,
            DirectoryTypes::Playlist(PlaylistMeta {}),
        );
        if res.is_ok() {
            return Ok("success".to_string());
        } else {
            return Err("failed to make playlist".to_string());
        }
    }
    pub fn edit_playlist(
        &mut self,
        playlist_name: String,
        video_paths: Vec<String>,
    ) -> Result<String, String> {
        let res = self.database.overwrite_link(
            playlist_name,
            video_paths,
            DirectoryTypes::Playlist(PlaylistMeta {}),
        );
        if res.is_ok() {
            return Ok("success".to_string());
        } else {
            return Err("failed to make playlist".to_string());
        }
    }
    pub fn get_playlist_all(&self, path_base: String, thumbnail_base: String) -> Vec<HtmlPlaylist> {
        let mut playlist_list = vec![];
        for (link, linked_keys) in self
            .database
            .iter_link_type(DirectoryTypes::Playlist(PlaylistMeta {})).ok().unwrap()
        {
            let mut vid_vec = vec![];
            for key in linked_keys {
                let vid_res = self.get_vid_html_from_path(
                    path_base.clone(),
                    thumbnail_base.clone(),
                    key.clone(),
                );
                if vid_res.is_ok() {
                    vid_vec.push(vid_res.ok().unwrap());
                }
            }
            playlist_list.push(HtmlPlaylist {
                videos: vid_vec,
                name: link,
            });
        }
        return playlist_list;
    }
    //gets the path of a video with a certain name
    pub fn get_vid_path(&self, name: String) -> Result<String, String> {
        for (_key, video) in self.database.iter_data().unwrap() {
            if video.name == name {
                return Ok(video.file_path.clone());
            }
        }
        return Err("video not found".to_string());
    }
    pub fn get_thumb_res(&self) -> Result<u32, String> {
        return Ok(self.thumb_res);
    }
    pub fn refresh(&mut self) -> Result<(), String> {
        let source = self.source_dir.clone();
        let db_path = self.database_path.clone();
        let play_before_join = self.get_playlist_all("foo".to_string(), "test".to_string());
        info!("checking database against file system");

        if source.is_some() && db_path.is_some() {
            let db_res = db_from_dir(
                source.unwrap(),
                self.thumb_dir.clone(),
                self.database_path.clone().unwrap(),
                self.thumb_res,
            );
            if db_res.is_ok() {
                let db = db_res.ok().unwrap();
                let join_res = self.database.right_join(db.database);

                if join_res.is_ok() {
                    let res = self.database.make_backed(db_path.unwrap());
                    if res.is_err() {
                        error!("Failed to write database to disk");
                        return Err("Failed to write database to disk".to_string());
                    }
                }
            } else {
                error!(
                    "failed to make database from directory: {}",
                    db_res.err().unwrap()
                );
                return Err(db_res.err().unwrap());
            }
        }
        for play in play_before_join {
            let mut vid_name_vec = vec![];
            for vid in play.videos {
                vid_name_vec.push(vid.path);
            }
        }
        info!("successfully reloaded database from disk");
        return Ok(());
    }
}
pub fn new(
    read_dir: String,
    thumb_dir: String,
    database_path: String,
    thumb_res: u32,
    num_recurse: u32,
) -> Result<VideoDB, String> {
    info!("creating backed datastructure");
    let make_db_res = gulkana::ServiceController::backed(database_path);

    if make_db_res.is_ok() {
        info!("made backed datastructure at path: {}", database_path);
        let make_db = make_db_res.ok().unwrap();
        assert!(Path::new(&database_path).exists());

        let mut video_db = VideoDB {
            database: make_db,
            database_path: Some(database_path),
            thumb_dir: thumb_dir,
            thumb_res: thumb_res,
            source_dir: Some(read_dir),
        };
        video_db.refresh()?;

        info!("starting to make thumbnails");
        let thumb_res = video_db.make_thumbnails();
        info!("finished making thumbnails");
        if thumb_res.is_ok() {
            return Ok(video_db);
        } else {
            return Err(thumb_res.err().unwrap());
        }
    } else {
        error!(
            "failed to make backed datastructure: {}",
            make_db_res.err().unwrap()
        );
        let parse_res = legacy_db::from_path(database_path.clone());

        if parse_res.is_ok() {
            let res = fs::remove_file(database_path.clone());
            if res.is_err() {
                return Err("failed to remove legacy db".to_string());
            }
            return from_legacy(
                parse_res.ok().unwrap(),
                read_dir,
                thumb_dir,
                database_path,
                thumb_res,
            );
        } else {
            let parse_res_str = parse_res.err().unwrap();
            error!(
                "Legacy Database Corrupted: {} Trying to delete database and rebuild",
                parse_res_str
            );

            let res = std::fs::remove_file(database_path.clone());
            if num_recurse < 2 && res.is_ok() {
                return new(
                    read_dir,
                    thumb_dir,
                    database_path,
                    thumb_res,
                    num_recurse + 1,
                );
            } else {
                return Err(format!("Legacy Database Corrupted: {} ", parse_res_str));
            }
        }
    }
}
fn db_from_dir(
    read_dir: String,
    _thumb_dir: String,
    _database_path: String,
    _thumb_res: u32,
) -> Result<VideoDB, String> {
    info!("making database from directory: {}", read_dir);
    let dir_iter_res = Path::new(&read_dir).read_dir();
    if dir_iter_res.is_ok() {
        let mut db = empty();
        for file in dir_iter_res.unwrap() {
            if file.is_ok() {
                let file_final = file.unwrap();
                let final_path = file_final.path();
                let file_name = file_final.file_name().into_string().unwrap();
                let file_path = final_path.clone().into_os_string().into_string().unwrap();
                let mut extension = "".to_string();
                let file_ext_res = final_path.extension();
                if file_ext_res.is_some() {
                    extension = file_ext_res.unwrap().to_str().unwrap().to_string();
                }

                let mut vid = FileData {
                    file_name: file_name.clone(),
                    file_path: file_path.clone(),
                    file_type: FileTypes::Unknown,
                    extension: extension,
                    name: file_name,
                    metadata: new_metadata(),
                };
                vid.gen_file_type();
                db.add_video(file_path, vid)?;
            } else {
                error!("file in directory invalid");
            }
        }

        return Ok(db);
    } else {
        error!("path: {} not directory", read_dir);
        return Err("path not directory".to_string());
    }
}
pub fn from_legacy(
    legacy_db: legacy_db::FileDB,
    read_dir: String,
    _thumb_dir: String,
    _database_path: String,
    _thumb_res: u32,
) -> Result<VideoDB, String> {
    let db_res = new(read_dir, _thumb_dir, _database_path, _thumb_res, 0);
    if db_res.is_ok() {
        let mut db = db_res.ok().unwrap();
        for vid in legacy_db.iter() {
            db.add_video(vid.file_name.clone(), FileData::from(vid.clone()))?;
        }
        let playlists = legacy_db.get_playlist_all();
        for play in playlists {
            db.add_playlist(play.name, play.video_paths)?;
        }

        db.refresh()?;
        return Ok(db);
    } else {
        return Err("failed to create database".to_string());
    }
}
pub fn empty() -> VideoDB {
    return VideoDB {
        database: gulkana::ServiceController::empty(),
        database_path: None,
        thumb_dir: "".to_string(),
        thumb_res: 0,
        source_dir: None,
    };
}
#[cfg(test)]
mod test {
    use std::fs::File;
    use super::*;
    #[test]
    fn build_database_empty(){
        let db = empty();
    }
}
