use serde::{Deserialize,Serialize};
use std::path::Path;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::{Arc,Mutex,RwLock};

mod thumbnail;
mod db;
#[derive(Clone,Serialize,Deserialize)]
pub struct VideoHtml{
    pub name: String,
    pub url: String,
    pub thumbnail_url: String,
    pub html_url:String,
}
#[derive(Clone)]
pub struct VideoDB{
    database: db::FileDB,
    thumb_dir:String,
    thumb_res:u32,
}
impl VideoDB{
    fn make_thumbnails(&mut self)->Result<String,String>{
        for file in self.database.iter_mut(){
            //make thumbnail 
            if file.is_video(){
                let thumb = thumbnail::make_thumb(file.file_path.clone(),
                    self.thumb_dir.clone(),self.thumb_res.clone());
                file.metadata=db::Metadata{thumbnail_path:thumb.path,thumbnail_name:thumb.name,thumbnail_res:thumb.resolution};
                
            }
        }
        return Ok("sucessfully made thumbnails".to_string());
    }
    pub fn get_vid_html_vec(&self,path_base:String,thumbnail_base:String)->Vec<VideoHtml>{
        let mut vec_out:Vec<VideoHtml>=Vec::new();
        for file in self.database.iter(){
            if file.is_video(){
                let name = file.name.clone();
                let mut url = path_base.clone();
                url.push_str(&name);

                let mut thumbnail_name=thumbnail_base.clone();
                thumbnail_name.push_str(&file.metadata.thumbnail_name.clone());
                vec_out.push(VideoHtml{name:file.name.clone(),
                    url:url.clone(),thumbnail_url:thumbnail_name,
                    html_url:url.clone()});
            }
        }
        return vec_out;
    }
    pub fn get_vid_html(&self,path_base:String,thumbnail_base:String,
            vid_name:String)->Result<VideoHtml,String>{
        for file in self.database.iter(){
            if file.name==vid_name{

            let name = file.name.clone();
            let mut url = path_base;
            url.push_str(&name);

            let mut thumbnail_name=thumbnail_base.clone();
            thumbnail_name.push_str(&file.metadata.thumbnail_name.clone());
            return Ok(VideoHtml{name:file.name.clone(),url:url.clone(),thumbnail_url:thumbnail_base,
                html_url:url});
            }
        }
        return Err("video not found".to_string());

    }
    //gets the path of a video with a certain name
    pub fn get_vid_path(&self,name:String)->Result<String,String>{
        for file in self.database.iter(){
            if file.name==name{
                return Ok(file.file_path.clone()); 
            }
        }
        return Err("video not found".to_string());
    }
    pub fn iter(&self)->std::slice::Iter<'_,db::FileData>{
        return self.database.iter();
    }
}
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
}
pub fn new(read_dir:String,thumb_dir:String,database_path:String,thumb_res:u32)->Result<VideoDB,String>{
    let make_db = db::new(database_path,read_dir);
    if make_db.is_ok(){
        let mut video_db=VideoDB{database:make_db.ok().unwrap(),thumb_dir:thumb_dir,thumb_res:thumb_res};
        let thumb_res = video_db.make_thumbnails();
        if thumb_res.is_ok(){
            return Ok(video_db);
        }else{
            return Err(thumb_res.err().unwrap());
        }
    }else{
        return Err(make_db.err().unwrap());
    }
}
pub fn empty()->VideoDB{
    return VideoDB{database:db::empty(),thumb_dir:"".to_string(),thumb_res:0};
}
