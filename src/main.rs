#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
mod state;
fn main(){
    //config::load_config();
    //videos::get_videos("videos".to_string());
    //webserver::setup_webserver();
    state::init();
}
