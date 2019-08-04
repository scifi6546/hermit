use argon2::{self,Config};

#[derive(Clone)]
pub struct User{
    pub name: String,
    pub password: String,
    pub token: String 
}
#[derive(Clone)]
pub struct UserVec{
    _users:Vec<User>
}
#[derive(Clone)]
pub struct UserConf{
    pub username: String,
    pub password: String,
}
impl UserVec{
    pub fn add_user(&mut self,username:String,password:String){
        let config=Config::default();
        let hash=argon2::hash_encoded(&password.into_bytes(),
            &get_salt(),&config).unwrap();

        let user_temp:User = User{name:username,
            password:hash,token:"".to_string()};

        self._users.push(user_temp);
    }
    pub fn load_user(&mut self,username:String,hashed_password:String)->Result<String,String>{
        let user_temp = User{name:username,password:hashed_password,token:"".to_string()};
        self._users.push(user_temp);
        return Ok("sucess".to_string());
    }
    pub fn logout(&mut self,token:String)->Result<String,String>{
        for i in 0..self._users.len(){
            if self._users[i].token==token{
                self._users[i].token="".to_string();
                return Ok("success".to_string());
            }
        }
        return Err("user not found".to_string());
    }
    //if  verification is sucessfull returns string with token if failed returns error message
    pub fn verify_user(&mut self,username:String,password:String)->Result<String,String>{
        for i in 0..self._users.len(){
            if self._users[i].name==username{
                if argon2::verify_encoded(&self._users[i].password,
                        &password.clone().into_bytes()).unwrap(){
                    println!("user sucessfully verified");
                    self._users[i].token=self.make_token();
                    return Ok(self._users[i].token.clone());
                }
                else{
                    println!("user not verified");
                }
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
        for user in self._users.clone(){
            if user.token==token{
                //returning new random token
                return self.make_token();
            }
        }
        return token;
    }
    //verifies a token
    pub fn verify_token(&self,token:String)->bool{
        if token==""{
            return false;
        }
        for user in self._users.clone(){
            if user.token==token{
                return true;
            }
        } 
        return false;

    }
    pub fn get_token(&self,username:String)->Result<String,String>{
        for user in self._users.clone(){
            if username==user.name{
                return Ok(user.token);
            }
        }
        return Err("user not found".to_string());
    }
    //checks if the structer is empty
    pub fn is_empty(&self)->bool{
        if self._users.is_empty(){
            return true;
        }
            return false;
    }
    pub fn print_users(&self)->String{
        let mut out:String=String::new();
        out.push_str("start users");
        for user in self._users.clone(){
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
        for user in self._users.clone(){
            vec_out.push(UserConf{
                username:user.name,
                password:user.password
                })
        }
        return vec_out;
    }
}
pub fn new()->UserVec{
    return UserVec{_users:[].to_vec()}; 
}
fn get_salt()->[u8;20]{
    let mut array:[u8;20]=[0;20];
    for i in 0..20{
        array[i] = rand::random::<u8>();
    }
    return array;
}
