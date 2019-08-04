use std::fs::{self};
use serde::{Deserialize,Serialize};
use std::path::Path;
use std::thread;
use std::sync::mpsc::channel;

mod thumbnail;
#[derive(Clone)]
pub struct Video{
    path: String,
    pub name: String,
    thumbnail_path: String,
    pub thumbnail_name: String,
}
#[derive(Clone,Serialize,Deserialize)]
pub struct VideoHtml{
    pub name: String,
    pub url: String,
    pub thumbnail_url: String,
    pub html_url:String,
}
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
    pub fn get_vid_html(&self,path_base:String,thumbnail_base:String)->VideoHtml{
        return VideoHtml{
            name:self.name.clone(),
            url:self.get_url(path_base.clone()),
            thumbnail_url: self.get_thumb(thumbnail_base),
			html_url:self.get_url(path_base),
        };
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
pub fn get_videos(read_dir:String,thumb_dir:String)->Vec<Video>{
    let path=Path::new(&read_dir);
    let thumb_path=Path::new(&thumb_dir);
    assert!(path.is_dir());
    assert!(thumb_path.is_dir());

    println!("looking for videos");
    let path=Path::new(&read_dir);
    let mut out_vid:Vec<Video>=Vec::new();
    //Todo make thumbnail creation run in parallel
    
    for entry in fs::read_dir(path).unwrap(){
        let entry = entry.unwrap();
        //let foo = channel();
        //foo.bar();
        let path_str:String = entry.path().to_str().unwrap().to_string();
        if is_video(path_str){
            let vid = make_thumbnail(entry,read_dir.clone(),thumb_dir.clone());
            out_vid.push(vid);
        }

        println!("file found");
    }
    print_videos(out_vid.clone());
    return out_vid;
}
fn make_thumbnail(video_entry: std::fs::DirEntry, vid_dir:String,thumb_dir:String)->Video{
    let vid_path_temp:&Path=Path::new(vid_dir.as_str());
    let vid_path = vid_path_temp.join(video_entry.file_name().to_str().unwrap());
    let thumb_info = thumbnail::make_thumb(vid_path.to_str().unwrap().to_string(),thumb_dir.clone()).clone();
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
