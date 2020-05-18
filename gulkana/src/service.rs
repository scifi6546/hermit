use crate::errors;
use crate::{backed_datastructure, new_datastructure, DataStructure};
use futures::executor::block_on;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{channel, Receiver, Sender};
///Service for use Database Side
pub struct ServiceDB<
    Key: std::marker::Sync + std::marker::Send,
    DataType: std::marker::Sync + std::marker::Send,
    LinkType: std::marker::Sync + std::marker::Send,
> {
    send_result: Sender<CommandResult<Key, DataType, LinkType>>,
    recieve_commands: Receiver<Command<Key, DataType, LinkType>>,
}
impl<
        Key: std::marker::Sync + std::marker::Send,
        DataType: std::marker::Sync + std::marker::Send,
        LinkType: std::marker::Sync + std::marker::Send,
    > ServiceDB<Key, DataType, LinkType>
{
    fn get_command(&mut self) -> Option<Command<Key, DataType, LinkType>> {
        let c = self.recieve_commands.try_recv();
        if c.is_ok() {
            Some(c.ok().unwrap())
        } else {
            None
        }
    }
    fn send_command_result(&mut self,res: CommandResult<Key,DataType,LinkType>){
        self.send_result.send(res);
    }
}
///Service used on Client Side
pub struct ServiceClient<
    Key: std::marker::Sync + std::marker::Send,
    DataType: std::marker::Sync + std::marker::Send,
    LinkType: std::marker::Sync + std::marker::Send,
> {
    send_commands: Sender<Command<Key, DataType, LinkType>>,
    recieve_result: Receiver<CommandResult<Key, DataType, LinkType>>,
}
impl<
        Key: std::marker::Sync + std::marker::Send,
        DataType: std::marker::Sync + std::marker::Send,
        LinkType: std::marker::Sync + std::marker::Send,
    > ServiceClient<Key, DataType, LinkType>
{
    fn send_command(
        &mut self,
        command: Command<Key, DataType, LinkType>,
    ) -> CommandResult<Key, DataType, LinkType> {
        self.send_commands.send(command);
        return self.recieve_result.recv().ok().unwrap();
    }
    fn get_result(&mut self) -> CommandResult<Key, DataType, LinkType> {
        self.recieve_result.recv().ok().unwrap()
    }
    pub fn quit(&mut self){
        self.send_command(Command::Quit);
    }
    /// Inserts data into datastructure
    /// ```
    ///  let mut s = gulkana::ServiceController::<u32,u32,u32>::empty();
    ///  s.insert(0,0);
    ///  assert_eq!(s.get(0).ok().unwrap(),0);
    ///  s.quit();
    /// ```
    pub fn insert(
        &mut self,
        key: Key,
        data: DataType,
    ) -> Result<(), errors::DBOperationError> {
        self.send_command(Command::Insert(key, data));
        Ok(())
    }
    ///Used to insert a link into a datastructure
    ///```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert_link(&9,&vec![10],0);
    /// let iter = ds.iter_links(&9).ok().unwrap();
    ///
    /// for (i,j) in iter{
    ///     assert!(*j==5);
    /// }
    ///```
    pub fn insert_link(
        &mut self,
        key: &Key,
        children: &std::vec::Vec<Key>,
        link_type: LinkType,
    ) -> Result<(), errors::DBOperationError> {
        Ok(())
    }
    ///Overwrites Links with vec shown
    ///```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert(11,6);
    /// ds.insert_link(&9,&vec![10],0);
    /// ds.overwrite_link(&9,&vec![11],0);
    /// ds.overwrite_link(&8,&vec![10],0);
    /// let iter = ds.iter_links(&9).ok().unwrap();
    ///
    /// for (_key,data) in iter{
    ///     assert!(*data==6);
    /// }
    /// let iter2 = ds.iter_links(&8).ok().unwrap();
    ///
    /// for (_key,data) in iter2{
    ///     assert!(*data==5);
    /// }
    /// ````
    pub fn overwrite_link(
        &mut self,
        key: &Key,
        children: &std::vec::Vec<Key>,
        link_type: LinkType,
    ) -> Result<(), errors::DBOperationError> {
        Ok(())
    }
    /// sets data in database
    /// ```
    /// let mut ds =gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,3);
    /// ds.set_data(&10,&5);
    /// assert!(ds.get(10).ok().unwrap()==5);
    /// ```
    pub fn set_data(&mut self, key: &Key, data: &DataType) -> Result<(), errors::DBOperationError> {
        Ok(())
    }
    /// Used to iterate through data
    ///
    pub fn iter_data(&self) {
        ()
    }
    /// Gets All keys in database
    ///
    /// ```
    ///
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// let out = ds.get_keys();
    /// assert!(out[0]==10);
    pub fn get_keys(&self) -> std::vec::Vec<Key> {
        vec![]
    }
    /// gets key from database
    pub fn get(&mut self, key: Key) -> Result<DataType, errors::DBOperationError> {
        let res = self.send_command(Command::GetKeys(key));
        match res{
            CommandResult::Get(data)=>Ok(data),
            _ =>Err(errors::DBOperationError::BrokenPipe)

        }
        
    }
    /// Gets linked nodes
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert(11,6);
    /// ds.insert_link(&9,&vec![10],0);
    /// let v = ds.get_links(&9).ok().unwrap();
    /// assert!(v[0]==10);
    /// ````

    pub fn get_links(&self, key: &Key) -> Result<Vec<Key>, errors::DBOperationError> {
        Ok(vec![])
    }

    /// Iterates through nodes attached to link
    ///
    pub fn iter_links(&self, key: &Key) -> Result<(), errors::DBOperationError> {
        Ok(())
    }
    /// Checks if database contains a given key
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// assert!(ds.contains(&10));
    /// assert!(!ds.contains(&20));
    /// ```
    pub fn contains(&self, key: &Key) -> bool {
        false
    }
    /// Gets iterator of links with labels
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert_link(&9,&vec![10],0);
    /// for (link,linked_keys) in ds.iter_link_type(&0){
    ///         assert!(link==9);
    /// }
    /// ```
    pub fn iter_link_type(&self, link_type: &LinkType) -> ()
    where
        LinkType: std::cmp::PartialEq,
    {
    }
    pub fn append_links(
        &mut self,
        key: &Key,
        key_append: &Key,
    ) -> Result<(), errors::DBOperationError> {
        Err(errors::DBOperationError::NodeNotLink)
    }
    pub fn right_join(
        &self,
        right: &DataStructure<Key, DataType, LinkType>,
    ) -> Result<DataStructure<Key, DataType, LinkType>, errors::DBOperationError>
    where
        Key: std::clone::Clone + std::cmp::Ord + Serialize,
        DataType: std::clone::Clone + Serialize,
        LinkType: std::clone::Clone + Serialize,
    {
        Err(errors::DBOperationError::NodeNotLink)
    }
    pub fn to_string(&self) -> Result<std::string::String, errors::SerializeError>
    where
        Key: Serialize,
        DataType: Serialize,
        LinkType: Serialize,
    {
        Ok("test".to_string())
    }
    /// Makes the database backed
    ///
    pub fn make_backed(&mut self, file_backing: &String) -> Result<(), errors::DBOperationError> {
        Ok(())
    }
    /// Gets number of elements in db
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// assert!(ds.len()==0);
    /// ds.insert(&20,20);
    /// assert!(ds.len()==1);
    /// ```
    pub fn len(&self) -> usize {
        0
    }
}
/// Used to send commands in between the Client and Master databases
#[derive(std::fmt::Debug, std::cmp::PartialEq)]
enum Command<Key: std::marker::Send, DataType: std::marker::Send, LinkType: std::marker::Send> {
    GetKeys(Key),
    Insert(Key, DataType),
    GetLinkTypeNOT_USED(LinkType),
    //Used to send Quit service to database
    Quit,
}
enum CommandResult<
    Key: std::marker::Sync + std::marker::Send,
    DataType: std::marker::Sync + std::marker::Send,
    LinkType: std::marker::Sync + std::marker::Send,
> {
    InsertOk,
    Get(DataType),
    Quit,
    Error(errors::DBOperationError),
    /// ************************************************************************
    ///  ***********************************************************************
    ///  ***********************************************************************
    /// FIX NOW!!!!!!!
    /// ************************************************************************
    /// ************************************************************************
    /// ************************************************************************
    /// ************************************************************************
    MakeCompillerHappy(Key,DataType,LinkType)
}
///Holds Database and Access to Services
pub struct ServiceController<
    Key: 'static+ std::marker::Sync + std::marker::Send + std::clone::Clone + Serialize + std::cmp::Ord,
    DataType: 'static+ std::marker::Sync + std::marker::Send + std::clone::Clone + Serialize,
    LinkType: 'static+ std::marker::Sync + std::marker::Send + std::clone::Clone + Serialize,
> {
    db: DataStructure<Key, DataType, LinkType>,
    service: Vec<ServiceDB<Key, DataType, LinkType>>,
}
impl<
        Key: 'static+std::marker::Sync
            + std::marker::Send
            + std::clone::Clone
            + Serialize
            + std::cmp::Ord
            + DeserializeOwned,
        DataType: 'static+std::marker::Sync + std::marker::Send + std::clone::Clone + Serialize + DeserializeOwned,
        LinkType: 'static+std::marker::Sync + std::marker::Send + std::clone::Clone + Serialize + DeserializeOwned,
    > ServiceController<Key, DataType, LinkType>
{
    pub fn backed(
        path: String,
    ) -> Result<ServiceClient<Key, DataType, LinkType>, crate::errors::DBOperationError> {
        let c:fn(String)-> ServiceController<Key, DataType, LinkType>= |path| {ServiceController {
            db: backed_datastructure(&path).ok().unwrap(),
            service: vec![],
        }};
        Ok(Self::make_controller_thread(c,path))
    }
    pub fn empty() -> ServiceClient<Key, DataType, LinkType> {
        Self::make_controller_thread(|()|{ServiceController {
            db: new_datastructure(),
            service: vec![],
        }},())
    }
    /// The main thread of the program
    fn main_loop(&mut self) {
        loop {
            let mut quit = false;
            let mut command_vec = vec![];
            command_vec.reserve(self.service.len());
            for mut service in &mut self.service{
                command_vec.push(service.get_command());
            }
            let mut i=0;
            let mut res_vec = vec![];
            for command in command_vec{                
                if command.is_some(){
                    let res = self.process_task(command.unwrap());
                    let r = match res{
                        CommandResult::Quit=>true,
                        _ =>false,
                    };
                    if r==true{
                        quit=true;
                    }
                    res_vec.push((i,res))
                }
                i+=1;
            }
            for (i,res) in res_vec{
                self.service[i].send_command_result(res);
            }
            if quit{
                break;
            }
        }
    }
    fn process_task(&mut self,command: Command<Key,DataType,LinkType>)->CommandResult<Key,DataType,LinkType>{
        match command{
            Command::Quit=>CommandResult::Quit,
            Command::Insert(key,data)=>self.insert(key,data),
            Command::GetKeys(key)=>self.get(key),
            Command::GetLinkTypeNOT_USED(_link)=>CommandResult::InsertOk,
        }
    }
    fn make_controller_thread<Args:'static + std::marker::Send>(
        s: fn(Args) -> ServiceController<Key, DataType, LinkType>,
        args:Args
    ) -> ServiceClient<Key, DataType, LinkType> {
        let (send, recieve) = channel();
        std::thread::spawn(move || {
            let mut controller = s(args);
            send.send(controller.add_service());
            controller.main_loop();
        });
        return recieve.recv().ok().unwrap();
    }
    pub fn add_service(&mut self) -> ServiceClient<Key, DataType, LinkType> {
        let (db, client) = new_client();
        self.service.push(db);
        return client;
    }
    /// Inserts into database
    fn insert(&mut self,key:Key,data:DataType)->CommandResult<Key,DataType,LinkType>{
        let res = self.db.insert(&key, data);
        if res.is_ok(){
            return CommandResult::InsertOk
        }else{
            return CommandResult::Error(res.err().unwrap());
        }
    }
    fn get(&mut self,key:Key)->CommandResult<Key,DataType,LinkType>{
        let res = self.db.get(&key);
        if res.is_ok(){
            return CommandResult::Get(res.ok().unwrap().clone())
        }else{
            return CommandResult::Error(res.err().unwrap());
        }
    }
}
fn new_client<
    Key: std::marker::Sync + std::marker::Send,
    DataType: std::marker::Sync + std::marker::Send,
    LinkType: std::marker::Sync + std::marker::Send,
>() -> (
    ServiceDB<Key, DataType, LinkType>,
    ServiceClient<Key, DataType, LinkType>,
) {
    let (command_send, command_recieve) = channel();
    let (result_send, result_recieve) = channel();
    (
        ServiceDB {
            send_result: result_send,
            recieve_commands: command_recieve,
        },
        ServiceClient {
            send_commands: command_send,
            recieve_result: result_recieve,
        },
    )
}
mod test {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn service_smoke_test() {
        let (_db, _c) = new_client::<u32, u32, u32>();
    }
    #[test]
    fn service_send() {
        use std::{thread, time};
        let (mut db, mut c) = new_client::<u32, u32, u32>();
        let t = std::thread::spawn(move || {
            c.send_command(Command::GetKeys(0));
        });
        db.send_command_result(CommandResult::InsertOk);
        thread::sleep(time::Duration::from_millis(10));
        
        #[allow(unused_must_use)]
        let _r = t.join();
        assert_eq!(db.get_command().unwrap(), Command::GetKeys(0));
    }
    #[test]
    fn quit_service_controller(){
        let mut c = ServiceController::<u32,u32,u32>::empty();
        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(10));
        c.quit();
    }
    #[test]
    fn insert_and_get() {
        let mut c = ServiceController::<u32, u32, u32>::empty();
        c.insert(0, 0);
        let r = c.get(0);
        assert_eq!(r.ok().unwrap(), 0);
        c.insert(1, 1);
        let r = c.get(1);
        assert_eq!(r.ok().unwrap(), 1);
        c.quit();
    }
}
