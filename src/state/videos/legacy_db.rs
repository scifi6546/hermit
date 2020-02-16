use std::fs::File;
use serde::{Deserialize, Serialize};
const DB_VERSION:u16=0;

#[derive(Clone,Serialize,Deserialize)]
pub struct VideoData{
    pub star_rating:u32,//star rating (eg 5 or 4 stars)
    pub rating:String,//normal rating (eg pg, pg13)
    pub description:String,//Dexcription Of video
}
//Metadata struct is used to store data not directly related
//to the file such as thumbnail path
#[derive(Clone,Serialize,Deserialize)]
pub struct Metadata{
    pub thumbnail_name:String,
    pub thumbnail_path:String,
    pub thumbnail_res:u32,
    pub video_data:VideoData,
    //to add stuff
}
#[derive(Clone,Serialize,Deserialize)]
pub struct FileData{
    pub file_name: String,
    pub name: String,
    pub file_path:String,
    pub extension:String,
    pub metadata:Metadata,
}
impl FileData{
    pub fn is_video(&self)->bool{
        if self.extension=="m4v".to_string() || self.extension=="ogg".to_string() ||
            self.extension=="mp4".to_string(){
            
            return true;
        }else{
            return false;
        }
    }
}

//a playlist database
#[derive(Clone,Serialize,Deserialize)]
pub struct Playlist{
    pub video_paths:Vec<String>,//paths of all videos, path is a unique identifier
    pub name:String,//name of playlist
}
#[derive(Clone,Serialize,Deserialize)]
pub struct FileDB{
    files:Vec<FileData>,
    db_path:String,
    file_path:String,
    version:u16,
    playlist:Vec<Playlist>,
}
impl FileDB{
    pub fn get_playlist_all(&self)->Vec<Playlist>{
        return self.playlist.clone();
    }
    pub fn getPlaylist(&self,name:String)->Result<Playlist,String>{
        for play in self.playlist.clone(){
            if play.name==name{
                return Ok(play.clone());
            }
        }
        let out = format!("db.rs: playlist {} not found",name);
        return Err(out);
    }
    //checks if the path is real and if the file is actually a video
    fn is_video(&self,video_path:String)->bool{
        for entry in self.iter(){
            if entry.file_path==video_path{
                if entry.is_video(){
                    return true;
                }
            }
        }
        return false;
    }
    //makes a playlist
    //gets mutable iterator of FileData
    pub fn iter(&self)->std::slice::Iter<'_,FileData>{
        let iterator = self.files.iter();
        return iterator;
    }
    pub fn get_file_from_path(&self,path:String)->Result<FileData,String>{
        for file in self.iter(){
            if file.file_path==path{
                return Ok(file.clone());
            }
        }
        return Err(format!("db.rs: file path {} not found",path));
    }
    //compares to files on disk and updates internal record accordingly
    fn is_sorted(&self)->bool{
        let mut previous="".to_string();
        for file in self.files.clone(){
            if file.file_path<previous{
                return false;
            }
            previous=file.file_path.clone();
        }
        return true;
    }
    //sorts all files by the filename using quicksort
    pub fn sort_by_filename(&mut self){
        self.p_qsort(0,self.files.len()-1);

    }
    //part of the sorting routine
    fn p_qsort(&mut self,low:usize,high:usize){
        if low<high{
            let pivot=self.p_partition(low,high);
            self.p_qsort(low,pivot);
            self.p_qsort(pivot+1,high);
        }
    }
    fn p_partition(&mut self,low:usize,high:usize)->usize{
        let mut low = low;
        let mut high=high;
        let pivot:String=self.files[low+(high-low)/2].file_path.clone();
        loop{
            while self.files[low].file_path.clone()<pivot{
                low+=1;
            }
            while self.files[high].file_path.clone()>pivot{
                high-=1;
            }
            if low>=high{
                return high;
            }
            let temp=self.files[low].clone();
            self.files[low]=self.files[high].clone();
            self.files[high]=temp;
            low+=1;
            high-=1;


        }
    }
}
pub fn empty()->FileDB{
    return FileDB{files:[].to_vec(),db_path:"".to_string(),file_path:"".to_string(),version:0,
        playlist:[].to_vec()};
}
pub fn from_path(file_path:String)->Result<FileDB,String>{
    let file_res = File::open(file_path);
    if file_res.is_ok(){
        let db_res = serde_json::from_reader(file_res.ok().unwrap());
        if db_res.is_ok(){
            return Ok(db_res.ok().unwrap());
        }else{

            return Err("failed to parse file".to_string());
        }

    }else{
        return Err("failed to find file".to_string());

    }

}
