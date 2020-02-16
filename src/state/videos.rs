use serde::{Deserialize, Serialize};
use std::path::Path;
mod legacy_db;
use gulkana;
use std::fs;
mod thumbnail;
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
}
impl FileData {
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
impl From<legacy_db::FileData> for FileData {
    fn from(f_in: legacy_db::FileData) -> FileData {
        return FileData {
            file_name: f_in.file_name,
            name: f_in.name,
            file_path: f_in.file_path,
            extension: f_in.extension,
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
struct PlaylistMeta{

}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum DirectoryTypes {
    Directory,
    Playlist(PlaylistMeta),
}
#[derive(Clone)]
pub struct VideoDB {
    database: gulkana::DataStructure<String, FileData, DirectoryTypes>,
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
fn empty_video_rating() -> VideoData {
    return VideoData {
        star_rating: 0,
        rating: "".to_string(),
        description: "".to_string(),
    };
}
impl VideoDB {
    fn make_thumbnails(&mut self) -> Result<String, String> {
        let mut keys = vec![];
        for (key, _file) in self.database.iter_data() {
            keys.push(key.clone());
        }
        for key in keys {
            //make thumbnail
            let file_res = self.database.get(&key);
            if file_res.is_ok() {
                let mut file = file_res.ok().unwrap().clone();
                if file.is_video() {
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
                        self.database.set_data(&key, &file);
                    } else {
                        return Err(thumb_res.err().unwrap());
                    }
                }
            }
        }
        return Ok("sucessfully made thumbnails".to_string());
    }
    pub fn add_video(&mut self, file_name: String, video_data: FileData) -> Result<String, String> {
        let res = self.database.set_data(&file_name, &video_data);
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
        println!("starting to get videos");
        for (_key, file) in self.database.iter_data() {
            println!("grabbed video");
            if file.is_video() {
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
                println!("video_description: {}", video_data.description);
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
    pub fn get_vid_html(
        &self,
        path_base: String,
        thumbnail_base: String,
        vid_name: String,
    ) -> Result<VideoHtml, String> {
        for (_key, file) in self.database.iter_data() {
            if file.name == vid_name {
                let name = file.name.clone();
                let mut url = path_base;
                url.push_str(&name);

                let video_data = VideoData {
                    rating: file.metadata.video_data.rating.clone(),
                    star_rating: file.metadata.video_data.star_rating,
                    description: file.metadata.video_data.description.clone(),
                };
                let mut thumbnail_name = thumbnail_base.clone();
                thumbnail_name.push_str(&file.metadata.thumbnail_name.clone());
                return Ok(VideoHtml {
                    name: file.name.clone(),
                    url: url.clone(),
                    thumbnail_url: thumbnail_name,
                    html_url: url,
                    path: file.file_path.clone(),
                    video_data: video_data,
                });
            }
        }
        return Err("video not found".to_string());
    }
    pub fn get_vid_data(&self, vid_path: String) -> Result<VideoData, String> {
        let res = self.database.get(&vid_path.clone());
        if res.is_ok() {
            let vid = res.ok().unwrap();
            let out = VideoData {
                star_rating: vid.metadata.video_data.star_rating,
                rating: vid.metadata.video_data.rating.clone(),
                description: vid.metadata.video_data.description.clone(),
            };
            return Ok(out);
        } else {
            return Err(format!(
                "videos.rs get_vid_data: path {} not found",
                vid_path
            ));
        }
    }
    pub fn get_vid_html_from_path(
        &self,
        path_base: String,
        thumbnail_base: String,
        vid_path: String,
    ) -> Result<VideoHtml, String> {
        let res = self.database.get(&vid_path);
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
        let res = self.database.get(&path);
        if res.is_ok() {
            let mut data = res.ok().unwrap().clone();

            data.metadata.video_data = VideoData {
                rating: to_change_to.rating,
                star_rating: to_change_to.star_rating,
                description: to_change_to.description,
            };
            self.database.set_data(&path, &data);
            return Ok("success".to_string());
        } else {
            return Err("file not found".to_string());
        }
    }
    pub fn add_playlist(
        &mut self,
        playlist_name: String,
        video_paths: Vec<String>,
    ) -> Result<String, String> {
        let res =
            self.database
                .overwrite_link(&playlist_name, &video_paths, DirectoryTypes::Playlist(PlaylistMeta{}));
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
        let res =
            self.database
                .overwrite_link(&playlist_name, &video_paths, DirectoryTypes::Playlist(PlaylistMeta{}));
        if res.is_ok() {
            return Ok("success".to_string());
        } else {
            return Err("failed to make playlist".to_string());
        }
    }
    pub fn get_playlist_all(&self, path_base: String, thumbnail_base: String) -> Vec<HtmlPlaylist> {
        let mut playlist_list = vec![];
        for (link, linked_keys) in self.database.iter_link_type(&DirectoryTypes::Playlist(PlaylistMeta{})) {
            let mut vid_vec = vec![];
            for key in linked_keys {
                let vid_res =
                    self.get_vid_html_from_path(path_base.clone(), thumbnail_base.clone(),key.clone());
                if vid_res.is_ok() {
                    vid_vec.push(vid_res.ok().unwrap());
                }else{
                    println!("error: {}",vid_res.err().unwrap());
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
        for (_key, video) in self.database.iter_data() {
            if video.name == name {
                return Ok(video.file_path.clone());
            }
        }
        return Err("video not found".to_string());
    }
    pub fn iter(&self) -> gulkana::DataNodeIter<'_, std::string::String, FileData, DirectoryTypes> {
        return self.database.iter_data();
    }
    pub fn get_thumb_res(&self) -> Result<u32, String> {
        return Ok(self.thumb_res);
    }
    pub fn refresh(&mut self){
        let source = self.source_dir.clone();
        let db_path = self.database_path.clone();
        println!("prejoin contents");
        self.print_contents();
        let play_before_join = self.get_playlist_all("foo".to_string(), "test".to_string());
        if source.is_some() && db_path.is_some() {
            let db_res = db_from_dir(
                source.unwrap(),
                self.thumb_dir.clone(),
                self.database_path.clone().unwrap(),
                self.thumb_res,
            );
            if db_res.is_ok() {
                let db = db_res.ok().unwrap();
                let join_res = self.database.right_join(&db.database);

                if join_res.is_ok() {
                    let join = join_res.ok().unwrap();
                    println!("join contents");
                    for (key, data) in join.iter_data() {
                        println!("{:?}", data);
                    }
                    self.database = join;
                    self.database.make_backed(&db_path.unwrap());
                } else {
                    println!("join res is not ok");
                }
            }
        }
        for play in play_before_join{
            let mut vid_name_vec = vec![];
            for vid in play.videos{
                vid_name_vec.push(vid.path);
            }
            self.add_playlist(play.name,vid_name_vec);
        }
        self.print_contents();
    }
    pub fn print_contents(&self) {
        println!("data after refresh: ");
        for (_key, data) in self.database.iter_data() {
            println!("{:?}", data);
        }
    }
}
/*
fn is_video(path_str: String)->bool{
    let path = Path::new(&path_str);
    let ext_opt = path.extension();
    let mut extension = "".to_string();
    if ext_opt.is_some(){
        let foo = ext_opt.unwrap();
        extension=foo.to_str().unwrap().to_string();
    }
    if path.is_file() && (extension=="m4v".to_string() || extension=="ogg".to_string() || extension=="mp4".to_string()){
        return true;
    }else{
        return false;
    }
}*/
/*Todo read from string
 *
 *
 */
pub fn new(
    read_dir: String,
    thumb_dir: String,
    database_path: String,
    thumb_res: u32,
) -> Result<VideoDB, String> {
    println!("db path: {}", database_path);
    let make_db_res = gulkana::backed_datastructure(&database_path);
    if make_db_res.is_ok() {
        println!("made db sucessfully");
        let make_db = make_db_res.ok().unwrap();
        assert!(Path::new(&database_path).exists());
        
        

        let mut video_db = VideoDB {
            database: make_db,
            database_path: Some(database_path),
            thumb_dir: thumb_dir,
            thumb_res: thumb_res,
            source_dir: Some(read_dir),
        };
        println!("contents from disk:");
        video_db.print_contents();
        video_db.refresh();
        let thumb_res = video_db.make_thumbnails();
        if thumb_res.is_ok() {
            return Ok(video_db);
        } else {
            return Err(thumb_res.err().unwrap());
        }
    } else {
        let parse_res = legacy_db::from_path(database_path.clone());
        
        if parse_res.is_ok() {
            fs::remove_file(database_path.clone());
            return from_legacy(
                parse_res.ok().unwrap(),
                read_dir,
                thumb_dir,
                database_path,
                thumb_res,
            );
        } else {
            return Err("todo".to_string());
        }
    }
}
fn db_from_dir(
    read_dir: String,
    _thumb_dir: String,
    _database_path: String,
    _thumb_res: u32,
) -> Result<VideoDB, String> {
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
                println!("file: {:?}", file_name);

                let vid = FileData {
                    file_name: file_name.clone(),
                    file_path: file_path.clone(),
                    extension: extension,
                    name: file_name,
                    metadata: new_metadata(),
                };
                db.add_video(file_path, vid);
            }
        }
        db.print_contents();

        return Ok(db);
    } else {
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
    let mut db_res = new(read_dir, _thumb_dir, _database_path, _thumb_res);
    if db_res.is_ok() {
        let mut db = db_res.ok().unwrap();
        for vid in legacy_db.iter() {
            db.add_video(vid.file_name.clone(), FileData::from(vid.clone()));
        }
        let playlists = legacy_db.get_playlist_all();
        for play in playlists {
            db.add_playlist(play.name, play.video_paths);
        }
        db.refresh();
        return Ok(db);
    } else {
        return Err("failed to create database".to_string());
    }
}
pub fn empty() -> VideoDB {
    return VideoDB {
        database: gulkana::new_datastructure(),
        database_path: None,
        thumb_dir: "".to_string(),
        thumb_res: 0,
        source_dir: None,
    };
}
#[cfg(test)]
mod test {}
