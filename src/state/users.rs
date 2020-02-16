use argon2::{self,Config};
use serde::{Serialize};

use gulkana::DataStructure;
#[derive(Clone,Serialize)]
pub struct User{
    pub name: String,
    pub password: String,
    pub token: String 
}
type Username=String;
#[derive(Clone,Serialize)]
pub enum Usertypes {}
#[derive(Clone)]
pub struct UserVec{
    //key is username
    pub _users: DataStructure<Username,User,Usertypes>,
    //_users:Vec<User>
}
#[derive(Clone)]
pub struct UserConf{
    pub username: String,
    pub password: String,
}
impl UserVec{
    pub fn add_user(&mut self,username:String,password:String)->Result<String,String>{
        for (username_temp,_user) in self._users.iter_data(){
            if &username==username_temp{
                return Err("user already exists".to_string());
            }
        }
        let config=Config::default();
        let hash=argon2::hash_encoded(&password.into_bytes(),
            &get_salt(),&config).unwrap();

        let user_temp:User = User{name:username.clone(),
            password:hash,token:"".to_string()};

        let insert_res = self._users.insert(&username,user_temp);
        if insert_res.is_ok(){
            return Ok("success".to_string());
        }else{
            return Err("failed to insert".to_string());
        }
    }
    pub fn load_user(&mut self,username:String,hashed_password:String)->Result<String,String>{
        let user_temp = User{name:username.clone(),password:hashed_password,token:"".to_string()};
        let insert_res = self._users.insert(&username,user_temp);
        if insert_res.is_ok(){
            return Ok("success".to_string());
        }else{
            return Err("failed to insert".to_string());
        }
    }
    pub fn logout(&mut self,token:String)->Result<String,String>{
        let mut to_change = vec![];
        for (username,user) in self._users.iter_data(){

            if user.token==token{
                let mut user_out = user.clone();
                //let mut mut_user = user.clone();
                user_out.token="".to_string();
                to_change.push((username.clone(),user_out));
                //self._users.set_data(username,&mut_user);
            }
        }
        let updated = to_change.len()>0;
        for (username,user) in to_change{
            let res = self._users.set_data(&username,&user);
            if res.is_err(){
                return Err("failed to remove token".to_string());
            }
        }
        if updated{
                return Ok("success".to_string());

        }
        return Err("user not found".to_string());
    }
    /// if  verification is sucessfull returns string with token if failed returns error message
    pub fn verify_user(&mut self,username_in:String,password:String)->Result<String,String>{
        let user_res = self._users.get(&username_in.clone());
        if user_res.is_ok(){
            let mut user = user_res.ok().unwrap().clone();

            if argon2::verify_encoded(&user.password,
                &password.clone().into_bytes()).unwrap(){

                    
                let token = self.make_token();
                user.token=token.clone();
                let set_data = self._users.set_data(&username_in,&user);
                if set_data.is_ok(){
                    return Ok(token);
                }else{
                    return Err("failed to set data".to_string());
                }
            }else{
                return Err("password incorrect".to_string());
            }
        }
        return Err("auth failed".to_string());
    }
    //generates a valid token
    fn make_token(&self)->String{
        let token_len = 20;
        let mut token:String=String::new();
        token.reserve(token_len);
        for _i in 0..token_len{
            token.push(rand::random::<char>());
        }
        //making sure that token is not already used
        for (_username,user) in self._users.iter_data(){
            if user.token==token{
                //returning new random token
                return self.make_token();
            }
        }
        return token;
    }
    //verifies a token and makes sure user is authorized
    pub fn verify_token(&self,token:String)->bool{
        if token==""{
            return false;
        }
        for (_username,user) in self._users.iter_data(){
            if user.token==token{
                return true;
            }
        } 
        return false;

    }
    #[allow(dead_code)]
    pub fn get_token(&self,username_in:String)->Result<String,String>{
        for (_username,user) in self._users.iter_data(){
            if username_in==user.name{
                return Ok(user.token.clone());
            }
        }
        return Err("user not found".to_string());
    }
    //checks if the structer is empty
    #[allow(dead_code)]
    pub fn is_empty(&self)->bool{
        self._users.len()==0
    }
    pub fn print_users(&self)->String{
        let mut out:String=String::new();
        out.push_str("start users");
        for (_username,user) in self._users.iter_data(){
            out.push_str("username: ");
            out.push_str(&user.name);
            out.push_str("  password: ");
            out.push_str(&user.password);
            out.push('\n');
        }
        out.push_str("end users");
        return out;
    }
    pub fn ret_conf_users(&self)->Vec<UserConf>{
        let mut vec_out:Vec<UserConf> = Vec::new();
        for (_username,user) in self._users.iter_data(){
            vec_out.push(UserConf{
                username:user.name.clone(),
                password:user.password.clone()
                })
        }
        return vec_out;
    }
    pub fn iter(&self)->gulkana::DataNodeIter<'_, std::string::String,User,Usertypes>{
        return self._users.iter_data()
    }
}
pub fn new()->UserVec{

    return UserVec{_users:gulkana::new_datastructure()}; 
}
fn get_salt()->[u8;20]{
    let mut array:[u8;20]=[0;20];
    for i in 0..20{
        array[i] = rand::random::<u8>();
    }
    return array;
}
#[cfg(test)]
mod test{
    use crate::state::users::new; 
    #[test]
    fn add_user(){
        let mut users = new();
        assert!(users.add_user("user".to_string(),"hunter2".to_string()).is_ok());
        assert!(users.verify_user("user".to_string(),
            "hunter2".to_string()).is_ok());
    }
    #[test]
    fn user_cant_login(){
        let mut users = new();
        assert!(users.verify_user("user".to_string(),
            "hunter2".to_string()).is_err());
    }
    #[test]
    fn get_user_token(){
        let user = "user".to_string();
        let password = "hunter2".to_string();
        let mut users = new();
        assert!(users.add_user(user.clone(),password.clone()).is_ok());
        let res = users.verify_user(user.clone(),password.clone());
        assert!(res.is_ok());
        let token = res.ok().unwrap();
        assert!(users.verify_token(token));
    }
    #[test]
    fn login_with_blank(){
        let mut users = new();
        assert!(users.add_user("user".to_string(),"hunter2".to_string()).is_ok());
        assert!(!users.verify_token("".to_string()));
    }

}
