mod videos;
mod config;
use actix_web::{middleware::Logger, web,App,HttpResponse,HttpServer,Responder,Result};
use actix_session::{Session, CookieSession};
use std::sync::RwLock;
use log::debug;
use actix_files;
use tera::Tera;
use serde::{Serialize,Deserialize};
mod users;
#[derive(Clone)]
pub struct State{
    pub config_file: config::Config,
    pub video_array: Vec<videos::Video>,
    pub users: users::UserVec,
    pub setup_bool:bool,
}
impl State{
    //returns cookie if user is suscessfully authenticated
    pub fn auth_user(&mut self,username:String,password:String)->Result<String,String>{
        self.print_users();
        let auth_res = self.users.verify_user(username.clone(),password);
        if auth_res.is_ok(){
            return Ok(auth_res.unwrap());
        }
        return Err("invalid credentials".to_string())
    }
    pub fn is_auth(&self,token:String)->bool{
        return self.users.verify_token(token);
    }
    pub fn logout(&mut self,token:String)->Result<String,String>{
        return self.users.logout(token);
        //todo
    }
    pub fn add_user(&mut self,username:String,password:String,user_token:String)->Result<String,String>{
        if self.users.verify_token(user_token){
            return self._add_user(username,password);
        }
        return Err("not authorized".to_string());
    }
    fn _add_user(&mut self, username:String,password:String)->Result<String,String>{
        self.users.add_user(username,password);
        let res = self.write();
        return res;
    }
	pub fn get_videos(&self,user_token:String)->Result<Vec<videos::VideoHtml>,String>{
        if self.is_auth(user_token){ 
		    let mut out:Vec<videos::VideoHtml>=Vec::new();
        
		    for vid in self.video_array.clone(){
			    out.push(vid.get_vid_html("/vid_html/".to_string(),"/thumbnails/".to_string()));	

		    }
		    return Ok(out);
        }
        else{
		    return Err("not authorized".to_string());
        }
	}
	pub fn get_vid_html(&self,user_token:String,video_name:String)->Result<videos::VideoHtml,String>{
		if self.users.verify_token(user_token){
			for vid in self.video_array.clone(){
				if vid.name==video_name{
					return Ok(vid.get_vid_html("/videos/".to_string(),"/thumbnails/".to_string()));
				}
			}
			return Err("not found".to_string());
		}else{
			return Err("not authorized".to_string())
		}
	}
	pub fn get_vid_dir(&self)->String{
		return self.config_file.videos.video_path.clone();
	}
        pub fn get_thumb_dir(&self)->String{
            return self.config_file.videos.thumbnails.clone();
        }
        pub fn is_setup(&self)->bool{
            return self.setup_bool;
        }
        pub fn setup(&mut self,video_dir:String, 
                     username:String, 
                     password:String)->Result<String,String>{
            if self.is_setup(){
                return Err("already setup".to_string());
            }
            let reload_res = self.reload_server(video_dir);
            let add_user_res = self._add_user(username,password);
            if reload_res.is_ok() && add_user_res.is_ok(){
                self.setup_bool=true;
                return Ok("Sucess".to_string());
            }else{
                return Err("failed to add user".to_string());
            }

        }
        pub fn reload_server(&mut self,video_dir:String, 
                     )->Result<String,String>{
            self.config_file.videos.video_path=video_dir.clone();
            self.config_file.videos.thumbnails="thumbnails".to_string();
            self.video_array=videos::get_videos(video_dir.clone(),"thumbnails".to_string());
            return Ok("done".to_string());
        }
    pub fn print_users(&self){
        println!("Users: ");
        println!("{}",self.users.print_users());    
    }
	fn write(&mut self)->Result<String,String>{
		let temp_user = self.users.ret_conf_users();
		let mut users_write:Vec<config::User>=Vec::new();
		for user in temp_user{
			users_write.push(config::User{
				username: user.username,
				passwd: user.password
			});
		}
		self.config_file.users=users_write;
		let res = config::write_conf(self.config_file.clone());
                if res.is_ok(){
                    return Ok("sucess".to_string());;
                }else{
                    return Err("error in writing".to_string()); 
                }
	}
}
lazy_static!{
	pub static ref TERA: Tera = {
		let tera = compile_templates!("templates/**/*");
		tera
	};
}
fn init_state()->State{
    let temp_cfg=config::load_config();
    if temp_cfg.is_ok(){
        let cfg = temp_cfg.ok().unwrap();
        let vid_dir=cfg.videos.video_path.clone();

        let mut out=State{
            config_file: cfg.clone(),
            video_array: videos::get_videos(vid_dir,"thumbnails".to_string()),
            users: users::new(),
            setup_bool: true,
        };
        for user in cfg.users.clone(){
            let res = out.users.load_user(user.username,user.passwd);
            if res.is_err(){
                println!("failed to add user");
            }
        }

        return out;
    }
    println!("error: {}",temp_cfg.clone().err().unwrap());
    return empty_state();

}
//returns an empty state
fn empty_state()->State{
    return State{
        config_file: config::empty(),
        video_array: [].to_vec(),
        users: users::new(),
        setup_bool: false,
    }
}
pub fn run_webserver(state_in:&mut State){
    let video_dir = state_in.get_vid_dir();
    let thumb_dir= state_in.get_thumb_dir();
    let temp_state = RwLock::new(state_in.clone());
    let shared_state = web::Data::new(temp_state);

    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
	env_logger::init();
    HttpServer::new(move || {
        App::new().wrap(
            CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                    .secure(false)
            ).wrap( Logger::default())
			.register_data(shared_state.clone())
            .route("/api/login",web::post().to(login))
			.route("/api/videos",web::get().to(get_videos))
			.route("/api/add_user",web::post().to(add_user))
			.route("/vid_html/{name}",web::get().to(vid_html))
            .route("/settings",web::get().to(settings))
            .route("/", web::get().to(index))
            .route("/login",web::get().to(login_html))
            .route("/setup",web::get().to(setup))
            .route("/api/setup",web::post().to(api_setup))
            .route("/api/logout",web::post().to(logout_api))

            .service(actix_files::Files::new("/static","./static/"))
        	.service(actix_files::Files::new("/videos",video_dir.clone()))
            .service(actix_files::Files::new("/thumbnails",thumb_dir.clone()))
			
    })
    .bind("0.0.0.0:8088")
    .unwrap()
    .run()
    .unwrap();
}
pub fn init(){
    let mut state_struct = init_state();
    run_webserver(&mut state_struct);
}
#[derive(Deserialize)]
struct UserReq{
    username: String,
    password: String,
}
fn login(info: web::Json<UserReq>, data:web::Data<RwLock<State>>,session:Session)-> Result<String>{
    println!("Processed Username: {} Password: {}",info.username,info.password);
	let mut state_data=data.write().unwrap();
    let auth=state_data.auth_user(info.username.clone(),info.password.clone());
    if auth.is_ok(){
        println!("Authenticated Username: {} Password: {}",info.username,info.password);
        let token = auth.unwrap();
        println!("token: {}",token.clone());
        let res = session.set("token",token);
        if res.is_ok(){
            return Ok("logged in sucessfully".to_string());
        }else{
            return Ok("failed to set cookie".to_string());
        }
    }
    else{
        println!("Denied Username: {} Password: {}",info.username,info.password);
        return Ok("Login Failed".to_string());

    }
}
fn add_user(info:web::Json<UserReq>,data:web::Data<RwLock<State>>,session:Session)->Result<String>{
    let token = session.get("token").unwrap().unwrap();
    let username = info.username.clone();
    let password = info.password.clone();
    let mut state_data = data.write().unwrap();
    state_data.print_users();
    let res = state_data.add_user(username.clone(),password.clone(),token);
    if res.is_ok(){
        println!("Added Username: {} Password: {}",username,password);
        return Ok("sucess".to_string());
    }
    return Ok("failed".to_string());
}
fn get_videos(data:web::Data<RwLock<State>>,session:Session)->impl Responder{
	let token = session.get("token").unwrap().unwrap();
	let state_data = data.read().unwrap();
	let videos=state_data.get_videos(token);
	let out=serde_json::to_string(&videos).unwrap();
	return HttpResponse::Ok().body(out);	
}
#[derive(Serialize)]
struct Index{
	videos: Vec<videos::VideoHtml>
}
pub fn index(data:web::Data<RwLock<State>>, session:Session)->impl Responder{
        println!("getting token");
        let temp = session.get("token");
        let mut token:String="".to_string();
        if temp.is_ok(){
            let temp_token = temp.ok().unwrap();
            if temp_token.is_some(){
                token=temp_token.unwrap();
            }
        }
        println!("getting state data");
	let state_data = data.read().unwrap();
    let index_data = state_data.get_videos(token); 
    if index_data.is_ok(){
	    let index_data=Index{
	        videos:index_data.ok().unwrap()
	    };
	    let out_data = TERA.render("home.jinja2",&index_data);
	    if out_data.is_ok(){
		    return HttpResponse::Ok().body(out_data.unwrap());
	    }else{
		    println!("data not rendered");
	    }
    }
    else{
        return HttpResponse::TemporaryRedirect().header("location", "/login").finish();
    }

    HttpResponse::Ok().body("".to_string())
        
}
pub fn setup(data:web::Data<RwLock<State>>,session:Session)->impl Responder{
        let render_data = TERA.render("setup.jinja2",&EmptyStruct{}); 
        let state = data.read();
        if render_data.is_ok() && !state.unwrap().is_setup(){

	    return HttpResponse::Ok().body(render_data.unwrap());
        }
            return HttpResponse::TemporaryRedirect().header("Location","/setup").finish();
}
pub fn settings(data:web::Data<RwLock<State>>,session:Session)->impl Responder{
    let render_data=TERA.render("settings.jinja2",&EmptyStruct{});
    let token_res = session.get("token");
    if token_res.is_ok(){
        let state = data.read();
        if render_data.is_ok() && state.unwrap().is_auth(token_res.unwrap().unwrap()){
            return HttpResponse::Ok().body(render_data.unwrap());
        }else{
            return HttpResponse::TemporaryRedirect().header("Location","/login").finish();
        }
    }else{
        return HttpResponse::TemporaryRedirect().header("Location","/login").finish();
    }
}
#[derive(Serialize,Deserialize)]
struct SetupStruct{
    video_dir:String,
    username:String,
    password:String,
}
fn api_setup(info: web::Json<SetupStruct>, data:web::Data<RwLock<State>>,
             session:Session)->Result<String>{
    let mut state_data = data.write().unwrap();
    let res =  state_data.setup(info.video_dir.clone(),info.username.clone(),info.password.clone());
    if res.is_ok(){
        return Ok("Sucess".to_string());
    }else{
        return Ok(res.err().unwrap());
    }
}
fn logout_api(into: web::Json<EmptyStruct>,session:Session,data:web::Data<RwLock<State>>)->Result<String>{
    let mut state_data=data.write().unwrap();
    let token_res = session.get("token");
    if token_res.is_ok(){
        let token:String = token_res.ok().unwrap().unwrap();
        let final_res = state_data.logout(token);
            if final_res.is_ok(){
                return Ok("Sucess".to_string());
            }else{
                return Ok("failed to logout".to_string());
            }
    }else{
        return Ok("failed to get token".to_string());
    }
}
#[derive(Deserialize,Serialize)]
struct EmptyStruct{

}
pub fn login_html(data:web::Data<RwLock<State>>, session:Session) -> impl Responder{
    println!("ran redirect");
    let state_data = data.read().unwrap();
    let html = TERA.render("login.jinja2",&EmptyStruct{});
    if html.is_ok(){
        return HttpResponse::Ok().body(html.unwrap());
    }
    else{
        println!("failed to render body");
        return HttpResponse::InternalServerError().body("");
    }
}
pub fn vid_html(data:web::Data<RwLock<State>>,session:Session,path: web::Path<(String,)>)->HttpResponse{

	let token:String = session.get("token").unwrap().unwrap();
	let vid_name:String = path.0.clone();
	let state_data = data.write().unwrap();
	let vid_res = state_data.get_vid_html(token,vid_name.clone());
	if vid_res.is_ok(){

		let vid:videos::VideoHtml = vid_res.unwrap();
		let data=TERA.render("video.jinja2",&vid);
		if data.is_ok(){
			return HttpResponse::Ok().body(data.unwrap());
		}else{
			println!("did not process template correctly");
		}
	}
	else{
		println!("did not get video");
	}
	//then use videos.jinja2 to create the data and return it
		
    HttpResponse::Ok().body(vid_name)
}
