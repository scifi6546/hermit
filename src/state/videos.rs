use std::fs::{self};
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
/*
impl Video{
    pub fn get_url(&self,path_base:String)->String{
        let mut out = path_base.clone();
        out.push_str(&self.name.clone());
        return out;
    }
    pub fn get_thumb(&self,thumbnail_base: String)->String{
        let mut out:String = thumbnail_base.clone();
        out.push_str(&self.thumbnail_name.clone());
        return out;
    }
    pub fn get_path(&self)->String{
        return self.path.clone();
    }

    pub fn get_vid_html(&self,path_base:String,thumbnail_base:String)->VideoHtml{
        return VideoHtml{
            name:self.name.clone(),
            url:self.get_url(path_base.clone()),
            thumbnail_url: self.get_thumb(thumbnail_base),
			html_url:self.get_url(path_base),
        };
    }
}*/
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
                println!("todo add thumbnail");
                let thumb = thumbnail::make_thumb(file.file_path.clone(),
                    self.thumb_dir.clone(),self.thumb_res.clone());
                file.metadata.thumbnail_path=thumb[0].clone();
                file.metadata.thumbnail_name=thumb[1].clone();
            }
        }
        return Err("todo".to_string());
    }
    pub fn get_vid_html_vec(&self,path_base:String,thumbnail_base:String)->Vec<VideoHtml>{
        let mut vec_out:Vec<VideoHtml>=Vec::new();
        for file in self.database.iter(){
            let name = file.name.clone();
            let mut url = path_base.clone();
            url.push_str(&name);

            let mut thumbnail_name=thumbnail_base.clone();
            thumbnail_name.push_str(&file.metadata.thumbnail_name.clone());
            vec_out.push(VideoHtml{name:file.name.clone(),
                url:url.clone(),thumbnail_url:thumbnail_name,
                html_url:url.clone()});
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
        video_db.make_thumbnails();
        return Ok(video_db);
    }else{
        return Err(make_db.err().unwrap());
    }
}
pub fn empty()->VideoDB{
    return VideoDB{database:db::empty(),thumb_dir:"".to_string(),thumb_res:0};
}
//Need to delete this
/*
pub fn get_videos(read_dir:String,thumb_dir:String,thumb_res:u32)->Vec<Video>{
    let path=Path::new(&read_dir);
    let thumb_path=Path::new(&thumb_dir);
    assert!(path.is_dir());
    assert!(thumb_path.is_dir());

    println!("looking for videos");
    let path=Path::new(&read_dir);
    let mut out_vid:Vec<Video>=Vec::new();
    //Todo make thumbnail creation run in parallel
    let mut threads = Vec::new();    
    let mutex_videos=Mutex::new(Vec::new());
    let arc_videos=Arc::new(mutex_videos);
    for entry in fs::read_dir(path).unwrap(){
        let entry = entry.unwrap();
        //let foo = channel();
        //foo.bar();
        let path_str:String = entry.path().to_str().unwrap().to_string();
        if is_video(path_str){
            let mut arc_vec = Arc::clone(&arc_videos);
            let read_dir_arc = Arc::new(RwLock::new(read_dir.clone()));
            let thumb_dir_arc = Arc::new(RwLock::new(thumb_dir.clone()));
            let mut read_dir_arc_c = Arc::clone(&read_dir_arc);
            let mut thumb_dir_arc_c = Arc::clone(&thumb_dir_arc);
            threads.push(thread::spawn(move || {
                let temp = read_dir_arc_c.read();
                let read_dir_temp = (*read_dir_arc_c.read().unwrap()).clone();
                let thumb_dir_temp = (*thumb_dir_arc_c.read().unwrap()).clone();
                let vid = make_thumbnail(entry,read_dir_temp,thumb_dir_temp,thumb_res);
                let mut vid_vec = &mut arc_vec.lock().unwrap();
                vid_vec.push(vid);
            }));

        }

        println!("file found");
    }
    for thread_single in threads{
        thread_single.join();
    }
    
    let mut arc_vec = Arc::clone(&arc_videos);
    out_vid=arc_vec.lock().unwrap().clone();

    print_videos(out_vid.clone());
    return out_vid;
}
fn make_thumbnail(video_entry: std::fs::DirEntry, vid_dir:String,thumb_dir:String,resolution:u32)->Video{
    let vid_path_temp:&Path=Path::new(vid_dir.as_str());
    let vid_path = vid_path_temp.join(video_entry.file_name().to_str().unwrap());
    let thumb_info = thumbnail::make_thumb(vid_path.to_str().unwrap().to_string(),
        thumb_dir.clone(),resolution).clone();
    let mut vid = Video{path:"".to_string(),
        name:"".to_string(),
        thumbnail_path: thumb_info[0].clone(), 
        thumbnail_name: thumb_info[1].clone(),
        };
    vid.path=video_entry.path().to_str().unwrap().to_string();
    vid.name=video_entry.path().file_name().unwrap().to_str().unwrap().to_string();
    return vid
}
fn print_videos(videos:Vec<Video>){
    for vid in videos{
        println!("Videos: ");
        println!("  name: {}",vid.name);
        println!("  path: {}",vid.path);
        println!("  thumbnail: {}",vid.thumbnail_path);
    }
}
*/
