use std::sync::mpsc::{channel,Sender,Receiver};
use serde::{Serialize,Deserialize};
use crate::DataStructure;
///Service for use Database Side
pub struct ServiceDB<Key,DataType,LinkType>{
    send_result: Sender<CommandResult<Key,DataType,LinkType>>,
    recieve_commands: Receiver<Command<Key,DataType,LinkType>>,

}
impl<Key,DataType,LinkType> ServiceDB<Key,DataType,LinkType>{
    fn get_command(&mut self)->Option<Command<Key,DataType,LinkType>>{
        let c = self.recieve_commands.try_recv();
        if c.is_ok(){
            Some(c.ok().unwrap())
        }else{
            None
        }

    }
}
///Service used on Client Side
pub struct ServiceClient<Key,DataType,LinkType>{
    send_commands: Sender<Command<Key,DataType,LinkType>>,
    recieve_result: Receiver<CommandResult<Key,DataType,LinkType>>
}
impl<Key,DataType,LinkType> ServiceClient<Key,DataType,LinkType>{
    fn send_command(&mut self,command: Command<Key,DataType,LinkType>){
        self.send_commands.send(command);
    }
}
#[derive(std::fmt::Debug,std::cmp::PartialEq)]
enum Command<Key,DataType,LinkType>{
    GetKeys(Key),
    InsertNode(Key,DataType),
    GetLinkType(LinkType)
}
struct CommandResult<Key,DataType,LinkType>{
    key:Option<Key>,
    data:Option<DataType>,
    link:Option<LinkType>
}
///Holds Database and Access to Services
struct ServiceController<Key:std::clone::Clone+std::cmp::Ord+Serialize,DataType:std::clone::Clone+std::cmp::Ord+Serialize,LinkType:std::clone::Clone+std::cmp::Ord+Serialize>{
    db:DataStructure<Key,DataType,LinkType>,
    service:Vec<ServiceDB<Key,DataType,LinkType>>,
}
impl<Key:std::clone::Clone+std::cmp::Ord+Serialize,DataType:std::clone::Clone+std::cmp::Ord+Serialize,LinkType:std::clone::Clone+std::cmp::Ord+Serialize> ServiceController<Key,DataType,LinkType>{
    //pub fn backed()->ServiceController<Key,DataType,LinkType>{
    //    
    //}
}

pub fn new_client<Key,DataType,LinkType>()->(ServiceDB<Key,DataType,LinkType>,ServiceClient<Key,DataType,LinkType>){
    let (command_send,command_recieve) = channel();
    let (result_send,result_recieve) = channel();
    (
        ServiceDB{
            send_result:result_send,
            recieve_commands:command_recieve,
        },
        ServiceClient{
            send_commands:command_send,
            recieve_result:result_recieve,
        }
    )
}
mod test{
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn service_smoke_test(){
        let (_db,_c) = new_client::<u32,u32,u32>();
    }
    #[test]
    fn service_send(){
        let (mut db,mut c) = new_client::<u32,u32,u32>();
        let t = std::thread::spawn(move || {
            c.send_command(Command::GetKeys(0));
        });
        #[allow(unused_must_use)]
        let _r = t.join();
        assert_eq!(db.get_command().unwrap(),Command::GetKeys(0));
    }
}