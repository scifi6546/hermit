use std::process::Command;
use std::path::Path;

pub struct ThumbData{
    pub path:String,
    pub name:String,
    pub resolution: u32,
}
//returns a vector of string 0th entry is path on disk of thumbnail (as in thumbnails/foo.png), 
//1st entry is thumbnail name (as in foo.png)
pub fn make_thumb(video_path:String,thumb_dir:String,resolution:u32)->ThumbData{
    //getting paths of video dir and thumbnail
    let vid_dir_path:&Path=Path::new(video_path.as_str());
    let thumb_dir_path:&Path = Path::new(thumb_dir.as_str());

    assert!(vid_dir_path.is_file());
    assert!(thumb_dir_path.is_dir());

    //getting video name
    let mut vid_name:String = vid_dir_path.file_name().unwrap().to_str().unwrap().to_string();

    //setting extension to .png so web browsers do not get confused
    vid_name.push_str(".png"); 

    //getting path on disk that thumbnail will be written too
    let thumb_path_final = thumb_dir_path.join(vid_name.as_str());
    println!("thumb_path_final: {}",thumb_path_final.to_str().unwrap());
    let thumb_path_str=thumb_path_final.to_str().unwrap().to_string();
    let thumb_comand = Command::new("ffmpegthumbnailer").args(&["-s",resolution.to_string().as_str(),"-i",vid_dir_path.to_str().unwrap(),
        "-o",thumb_path_final.to_str().unwrap()]).output();
    let output = thumb_comand.unwrap();
    println!("ffmpeg output: {}",String::from_utf8_lossy(&output.stdout));
    assert!(thumb_path_final.exists());
    return ThumbData{path:thumb_path_str,name:vid_name,resolution:resolution};

}
