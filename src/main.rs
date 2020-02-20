use std::env;
#[macro_use] extern crate log;
use log::Level;
mod state;
fn main(){
    env_logger::init();
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
    state::init(ssl);
}
