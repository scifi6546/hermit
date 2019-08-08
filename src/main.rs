#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
use std::env;
mod state;
fn main(){
    let mut ssl=true;
    let mut previous:String="".to_string();
    for arg in env::args(){
        if previous=="-ssl"{
            if arg=="yes"{
                ssl=true;
            }if arg=="no"{
                ssl=false; 
                println!("not using ssl!");
            }
        }
        previous=arg.to_string()
    }
    //config::load_config();
    //videos::get_videos("videos".to_string());
    //webserver::setup_webserver();
    state::init(ssl);
}
