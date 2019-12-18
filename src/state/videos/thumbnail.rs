use std::process::Command;
use std::path::Path;
use std::fs;

pub struct ThumbData{
    pub path:String,
    pub name:String,
    pub resolution: u32,
}
//returns a vector of string 0th entry is path on disk of thumbnail (as in thumbnails/foo.png), 
//1st entry is thumbnail name (as in foo.png)
pub fn make_thumb(video_path:String,thumb_dir:String,resolution:u32)->Result<ThumbData,String>{
    //getting paths of video dir and thumbnail
    //debug!("{}: opening directory: {} and file: {}","db.rs",thumb_dir,video_path);
    let vid_dir_path:&Path=Path::new(video_path.as_str());
    let thumb_dir_path:&Path = Path::new(thumb_dir.as_str());
    if !vid_dir_path.is_file(){
        return Err(format!("thumbnail.rs: vid_dir {} is not a file",vid_dir_path.display()));
    }
    if(!thumb_dir_path.exists()){
        fs::create_dir(thumb_dir_path.to_str().unwrap());

    }
    if !thumb_dir_path.is_dir(){

        return Err(format!("thumbnail.rs: thumb_dir_path {} is not a directory",vid_dir_path.display()));
    }

    //getting video name
    let mut vid_name:String = vid_dir_path.file_name().unwrap().to_str().unwrap().to_string();

    //setting extension to .png so web browsers do not get confused
    vid_name.push_str(".png"); 

    //getting path on disk that thumbnail will be written too
    let thumb_path_final = thumb_dir_path.join(vid_name.as_str());
    let thumb_path_str=thumb_path_final.to_str().unwrap().to_string();
    let thumb_comand = Command::new("ffmpegthumbnailer").args(&["-s",resolution.to_string().as_str(),"-i",vid_dir_path.to_str().unwrap(),
        "-o",thumb_path_final.to_str().unwrap()]).output();
    if thumb_comand.is_err(){
        return Err("thumbnail.rs: ffmpegthumbnailer failed to execute".to_string());
    }
    if !thumb_path_final.exists(){
        return Err("thunail.rs: thumbnail never written to disk".to_string());
    }

    assert!(thumb_path_final.exists());
    return Ok(ThumbData{path:thumb_path_str,name:vid_name,resolution:resolution});

}
