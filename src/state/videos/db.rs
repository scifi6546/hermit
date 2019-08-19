use std::path::Path;
use std::fs::File;
use std::io::{Write,Read};
use std::fmt;
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
fn new_metadata()->Metadata{
    return Metadata{thumbnail_path:"".to_string(),thumbnail_name:"".to_string(),
        thumbnail_res:0,video_data:VideoData{
            star_rating:0,
            rating: "".to_string(),
            description:"".to_string(),
        }}
}
fn file_data_from_path(file_path:String)->Result<FileData,String>{
    let file_path = Path::new(&file_path);
    if file_path.is_file(){
        let file_name:String=file_path.file_name().unwrap().to_str().unwrap().to_string();
        return Ok(
            FileData{file_name:file_name.clone(),
            name:file_name,
            file_path:file_path.to_str().unwrap().to_string(),
            extension:file_path.extension().unwrap().to_str().unwrap().to_string(),
            metadata:new_metadata()} 
        );
    }else{
        return Err("Not actual file".to_string());
    }
}
impl FileDB{
    fn write(&mut self)->Result<String,String>{
        let output = serde_json::to_string(self);
        if output.is_ok(){
        let file_res = File::create(self.db_path.clone());
            if file_res.is_ok(){
                let mut file = file_res.ok().unwrap();
                let write_res = file.write_all(output.ok().unwrap().as_bytes());
                if write_res.is_ok(){
                    return Ok("sucess".to_string());
                }else{
                    return Err("db.rs: failed to write to file".to_string());
                }
            }else{
                return Err("db.rs: failed to open database file".to_string());
            }
        }else{
            return Err("db.rs: failed to create output database file".to_string());
        }
    }
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
    pub fn add_playlist(&mut self,playlist_name:String,video_paths:Vec<String>)->Result<String,String>{
        for play in self.playlist.clone(){
            if play.name==playlist_name{
                return Err(format!("playlist with name {} already exists!",playlist_name));
            }
        }
        for vid in video_paths.clone(){
            if !self.is_video(vid.clone()){
                return Err(format!("vid with path {} does not exist",vid));
            }
        }
        self.playlist.push(Playlist{name:playlist_name,video_paths:video_paths});
        let res = self.write();
        if res.is_ok(){
            return Ok("success".to_string());
        }else{
            return Err(res.err().unwrap());
        }
    }
    //gets mutable iterator of FileData
    pub fn iter_mut(&mut self)->std::slice::IterMut<'_,FileData>{
        let iterator = self.files.iter_mut();
        return iterator;
    }
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
    //edits video data
    pub fn edit_video_data(&mut self,path:String,data:VideoData)->Result<String,String>{
        for vid in self.iter_mut(){
            if vid.file_path==path{
                vid.metadata.video_data=data;
                return Ok("db.rs edit_video_data(): video data set sucessfully".to_string());
            }
        }
        return Err(format!("db.rs edit_video_data(): video for path {} not found",path));
    }
    //compares to files on disk and updates internal record accordingly
    pub fn compare_disk(&mut self)->Result<String,String>{
        self.sort_by_filename();
        println!("{}",self.is_sorted());
        
        //getting all files in disk and putting them in vec file_vec
        let mut file_vec:Vec<String>=Vec::new();
        let read_dir=self.file_path.clone();
        let folder = Path::new(&read_dir);
        let fold_iterater=folder.read_dir();
        if fold_iterater.is_ok(){
            for file in fold_iterater.ok().unwrap(){
                if !file.is_ok(){
                    return Err("file not ok!".to_string())
                }
                let file_path:String=file.unwrap().path().into_os_string()
                    .into_string().unwrap();
                file_vec.push(file_path);
            }
            file_vec.sort();
        } 
        let mut temp_file_db:Vec<FileData>=Vec::new();
        //JOIN Algorithm:
        let mut db_iterator:usize=0;
        let mut fs_iterator:usize=0;
        loop{
            //checking both entries are the same, if they are add the entry in the database to a 
            //new one
            if db_iterator<self.files.len() && fs_iterator<file_vec.len(){
                println!("db_iterator: {}",db_iterator);
                println!("fs_iterator: {}",fs_iterator);
                println!("");
                //check if two entries are the same
                if self.files[db_iterator].file_path==file_vec[fs_iterator]{
                    //ADD self.files[db_iterator] to temp db
                    //iterating both db_iterator and fs_iterator because an entry in both vecs were
                    //used
                    temp_file_db.push(self.files[db_iterator].clone());
                    db_iterator+=1;
                    fs_iterator+=1;
                }//adding else if so that only one if statement can be executed per loop
                else if self.files[db_iterator].file_path<file_vec[fs_iterator]{
                    //this means that the current self.files element is not present in
                    //file_vec and skipping the self.files entry 
                    //not adding file_vec entry becuase it will be added when self.files==file_vec
                    //or when self.files>file_vec
                    db_iterator+=1;
                }
                else if self.files[db_iterator].file_path>file_vec[fs_iterator]{
                    //ADD file_vec to temp db
                    //file_vec is less then self.files
                    //this means that a element in file_vec needs to be added
                    let res = file_data_from_path(file_vec[fs_iterator].clone());
                    if res.is_ok(){
                        temp_file_db.push(res.ok().unwrap());
                    }
                    fs_iterator+=1;
                }
            }else{
                //if stepped of end of file system then no remaining entries in the db can be valid becase
                //they are not present in the fs
                if fs_iterator>=file_vec.len(){
                    break;
                }
                if db_iterator>=self.files.len(){
                    let res = file_data_from_path(file_vec[fs_iterator].clone());
                    if res.is_ok(){
                        temp_file_db.push(res.ok().unwrap());
                    }
                    //ADD file_vec[fs_iterator] to temp db
                    fs_iterator+=1;
                }
            }
        }
        self.files=temp_file_db;
        let res = self.write();
        if res.is_ok(){
            return Ok("database sucessfully generated".to_string());
        }
        return Err("failed to write database to disk".to_string());
    }
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
impl std::fmt::Display for FileDB{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_string:String = String::new();
        for file in self.files.clone(){
            out_string.push_str("file_name: ");
            out_string.push_str(file.name.as_str());
            out_string.push('\n');
        }
        return write!(f, "{}",out_string);
    }
}
pub fn new(database_path:String,file_path:String)->Result<FileDB,String>{
    let mut database:FileDB;
    if Path::new(&database_path).is_file(){
        println!("is file");
        let res = reload_databse(database_path.clone());
        if res.is_ok(){
            database = res.ok().unwrap();
        }else{
            let res =  create_new_db(database_path.clone(),file_path.clone());
            if res.is_ok(){
                database=res.ok().unwrap();
            }else{
                return res;
            }
        }
    }else{
        let res =  create_new_db(database_path.clone(),file_path.clone());
        if res.is_ok(){
            database=res.ok().unwrap();
        }else{
            return res;
        }
    }
    let res = database.write();
    if res.is_ok(){
        println!("{}",database);
        return Ok(database);
    }else{
        return Err(res.err().unwrap());
    }
}
pub fn empty()->FileDB{
    return FileDB{files:[].to_vec(),db_path:"".to_string(),file_path:"".to_string(),version:0,
        playlist:[].to_vec()};
}
fn create_new_db(database_path:String,file_path:String)->Result<FileDB,String>{
    let folder_path = Path::new(&file_path);
    let file_iterator = folder_path.read_dir();
    let mut files:Vec<FileData> = Vec::new();
    if file_iterator.is_ok(){
        for file in file_iterator.unwrap(){
            if file.is_ok(){
                let file_final=file.unwrap();
                let final_path=file_final.path();
                let file_name = file_final.file_name().into_string().unwrap();
                let file_path=final_path.clone().into_os_string().into_string().unwrap();
                let mut extension="".to_string();
                let file_ext_res = final_path.extension();
                if file_ext_res.is_some(){
                    extension=file_ext_res.unwrap().to_str().unwrap().to_string();
                }
                println!("file: {:?}",file_name);
                files.push(
                    FileData{file_name:file_name.clone(),file_path:file_path,
                    extension:extension,
                    name:file_name,metadata:new_metadata()}
                );
            }
        }
        return Ok(FileDB{files:files,db_path:database_path,file_path:file_path,version:DB_VERSION,
            playlist:[].to_vec()});
    }else{
        return Err("db.rs: Folder not found".to_string());
    }
}
fn reload_databse(database_path:String)->Result<FileDB,String>{
    let file_res = File::open(database_path.as_str());
    if file_res.is_ok(){
        let mut data_str:String = String::new();
        let res = file_res.ok().unwrap().read_to_string(&mut data_str);
        if res.is_err(){
            return Err("failed to read database into string".to_string());
        }
        println!("{}",data_str);
        let parse_res = serde_json::from_str::<FileDB>(data_str.as_str());
        if parse_res.is_ok(){
            let mut database:FileDB = parse_res.unwrap();
            //todo check if filesystem matches files
            let res = database.compare_disk();
            if res.is_ok(){
                return Ok(database);
            }else{
                return Err(res.err().unwrap());
            }
        }else{
            return Err("failed to parse database".to_string());
        }
    }else{
        return Err("Failed to open database file".to_string());
    }
}
