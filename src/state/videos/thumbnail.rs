use std::process::Command;
use std::path::Path;
//returns a vector of string 0th entry is path 1st entry is thumbnail name
pub fn make_thumb(vid_dir:String,thumb_dir:String)->Vec<String>{
    let vid_dir_path:&Path=Path::new(vid_dir.as_str());
    let thumb_dir_path:&Path = Path::new(thumb_dir.as_str());

    assert!(vid_dir_path.is_file());
    assert!(thumb_dir_path.is_dir());

    let mut vid_name:String = vid_dir_path.file_name().unwrap().to_str().unwrap().to_string();
    vid_name.push_str(".png"); 
    let thumb_path_final = thumb_dir_path.join(vid_name.as_str());
    println!("thumb_path_final: {}",thumb_path_final.to_str().unwrap());
    let thumb_path_str=thumb_path_final.to_str().unwrap().to_string();
    let thumb_comand = Command::new("ffmpegthumbnailer").args(&["-s","400","-i",vid_dir_path.to_str().unwrap(),
        "-o",thumb_path_final.to_str().unwrap()]).output();
    let output = thumb_comand.unwrap();
    println!("ffmpeg output: {}",String::from_utf8_lossy(&output.stdout));
    assert!(thumb_path_final.exists());
    println!("thumbnail_path: {}",thumb_path_str);
    return [thumb_path_str,vid_name].to_vec();

}
