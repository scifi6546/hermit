use crate::errors;
use crate::{backed_datastructure, new_datastructure, DataStructure};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::mpsc::{channel, Receiver, Sender};
mod iterators;
use iterators::*;
mod commands;
use commands::*;
///Service for use Database Side
pub struct ServiceDB<
    Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq,
    DataType: std::marker::Sync + std::marker::Send,
    LinkType: std::marker::Sync + std::marker::Send,
> {
    send_result: Sender<CommandResult<Key, DataType, LinkType>>,
    recieve_commands: Receiver<Command<Key, DataType, LinkType>>,
}
impl<
        Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq,
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
    fn send_command_result(&mut self, res: CommandResult<Key, DataType, LinkType>) {
        self.send_result.send(res);
    }
}
///Service used on Client Side
pub struct ServiceClient<
    Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq,
    DataType: std::marker::Sync + std::marker::Send,
    LinkType: std::marker::Sync + std::marker::Send,
> {
    send_commands: Sender<Command<Key, DataType, LinkType>>,
    recieve_result: Receiver<CommandResult<Key, DataType, LinkType>>,
}
impl<
        Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq,
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
    pub fn quit(&mut self) {
        self.send_command(Command::Quit);
    }
    /// Inserts data into datastructure
    /// ```
    ///  let mut s = gulkana::ServiceController::<u32,u32,u32>::empty();
    ///  s.insert(0,0);
    ///  assert_eq!(s.get(0).ok().unwrap(),0);
    ///  s.quit();
    /// ```
    pub fn insert(&mut self, key: Key, data: DataType) -> Result<(), errors::DBOperationError> {
        match self.send_command(Command::Insert(key, data)) {
            CommandResult::InsertOk => Ok(()),
            CommandResult::Error(e) => Err(e),
            _ => Err(errors::DBOperationError::Other),
        }
    }
    ///Used to insert a link into a datastructure
    ///```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert_link(9,vec![10],0);
    /// let iter = ds.iter_links(9).ok().unwrap();
    /// for (i,j) in iter{
    ///     assert!(j==5);
    /// }
    ///```
    pub fn insert_link(
        &mut self,
        key: Key,
        children: std::vec::Vec<Key>,
        link_type: LinkType,
    ) -> Result<(), errors::DBOperationError> {
        match self.send_command(Command::InsertLink(key, children, link_type)) {
            CommandResult::InsertOk => Ok(()),
            _ => Err(errors::DBOperationError::BrokenPipe),
        }
    }
    ///Overwrites Links with vec shown
    ///```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert(11,6);
    /// ds.insert_link(9,vec![10],0);
    /// ds.overwrite_link(9,vec![11],0);
    /// ds.overwrite_link(8,vec![10],0);
    /// let iter = ds.iter_links(9).ok().unwrap();
    ///
    /// for (_key,data) in iter{
    ///     assert!(data==6);
    /// }
    /// let iter2 = ds.iter_links(8).ok().unwrap();
    ///
    /// for (_key,data) in iter2{
    ///     assert!(data==5);
    /// }
    /// ````
    pub fn overwrite_link(
        &mut self,
        key: Key,
        children: std::vec::Vec<Key>,
        link_type: LinkType,
    ) -> Result<(), errors::DBOperationError> {
        match self.send_command(Command::OverwriteLink(key, children, link_type)) {
            CommandResult::InsertOk => Ok(()),
            CommandResult::Error(e) => Err(e),
            _ => Err(errors::DBOperationError::Other),
        }
    }
    /// sets data in database
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,3);
    /// ds.set_data(10,5);
    /// assert!(ds.get(10).ok().unwrap()==5);
    /// ```
    pub fn set_data(&mut self, key: Key, data: DataType) -> Result<(), errors::DBOperationError> {
        match self.send_command(Command::OverwriteData(key, data)) {
            CommandResult::InsertOk => Ok(()),
            CommandResult::Error(e) => Err(e),
            _ => Err(errors::DBOperationError::Other),
        }
    }
    /// Used to iterate through data
    /// Collects all data before sending iterator so on large databases a deep copy is made of the
    /// entire databse before sending the iterator
    /// ```
    ///  let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    ///  ds.insert(10,3);
    ///  for (key,data) in ds.iter_data().unwrap(){
    ///     assert_eq!(key,10);
    ///     assert_eq!(data,3);
    /// }
    /// ````
    pub fn iter_data(&mut self) -> Option<DataIter<(Key, DataType)>> {
        match self.send_command(Command::GetAllData) {
            CommandResult::ReturnAllData(v) => Some(DataIter::new(v)),
            _ => None,
        }
    }
    /// Gets All keys in database
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// let out = ds.get_keys().ok().unwrap();
    /// assert!(out[0]==10);
    /// ```
    pub fn get_keys(&mut self) -> Result<std::vec::Vec<Key>, errors::DBOperationError> {
        match self.send_command(Command::GetAllKeys) {
            CommandResult::GetAllKeys(k) => Ok(k),
            CommandResult::Error(e) => Err(e),
            _ => Err(errors::DBOperationError::Other),
        }
    }
    /// gets key from database
    pub fn get(&mut self, key: Key) -> Result<DataType, errors::DBOperationError> {
        let res = self.send_command(Command::GetKeys(key));
        match res {
            CommandResult::Get(data) => Ok(data),
            _ => Err(errors::DBOperationError::Other),
        }
    }
    /// Gets linked nodes
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert(11,6);
    /// ds.insert_link(9,vec![10],0);
    /// let v = ds.get_links(9).ok().unwrap();
    /// assert!(v[0]==10);
    /// ````

    pub fn get_links(&mut self, key: Key) -> Result<Vec<Key>, errors::DBOperationError> {
        match self.send_command(Command::GetLinkedKeys(key)) {
            CommandResult::GetLinkedKeys(v) => Ok(v),
            CommandResult::Error(e) => Err(e),
            _ => Err(errors::DBOperationError::Other),
        }
    }

    /// Iterates through nodes attached to link
    pub fn iter_links(
        &mut self,
        key: Key,
    ) -> Result<DataIter<(Key, DataType)>, errors::DBOperationError> {
        match self.send_command(Command::GetLinkedData(key)) {
            CommandResult::GetLinkedData(v) => Ok(DataIter::new(v)),
            CommandResult::Error(e) => Err(e),
            _ => Err(errors::DBOperationError::Other),
        }
    }
    /// Checks if database contains a given key
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// assert!(ds.contains(10).unwrap());
    /// assert!(!ds.contains(20).unwrap());
    /// ```
    pub fn contains(&mut self, key: Key) -> Option<bool> {
        match self.send_command(Command::GetContains(key)) {
            CommandResult::Contains(val) => Some(val),
            _ => None,
        }
    }
    /// Gets iterator of links with labels
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert_link(9,vec![10],0);
    /// for (link,linked_keys) in ds.iter_link_type(0).ok().unwrap(){
    ///         assert!(link==9);
    /// }
    /// ```
    pub fn iter_link_type(
        &mut self,
        link_type: LinkType,
    ) -> Result<DataIter<(Key, Vec<Key>)>, errors::DBOperationError>
    where
        LinkType: std::cmp::PartialEq,
    {
        match self.send_command(Command::IterLinkType(link_type)) {
            CommandResult::IterLinkType(a) => Ok(DataIter::new(a)),
            CommandResult::Error(e) => Err(e),
            _ => Err(errors::DBOperationError::Other),
        }
    }
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(10,5);
    /// ds.insert(11,5);
    /// ds.insert_link(9,vec![10],0);
    /// let v = ds.get_links(9).ok().unwrap();
    /// assert_eq!(v[0],10);
    /// ds.append_links(9,11);
    /// let v2 = ds.get_links(9).ok().unwrap();
    /// assert_eq!(v2,vec![10,11]);
    /// ```
    pub fn append_links(
        &mut self,
        key: Key,
        key_append: Key,
    ) -> Result<(), errors::DBOperationError> {
        Err(errors::DBOperationError::NodeNotLink)
    }
    /// Have Not figured out a good interface
    /// ```
    /// assert!(1==0);
    /// ```
    pub fn right_join(
        &self,
        right: fn(&mut ServiceClient<Key,DataType,LinkType>)->Result<(),errors::DBOperationError>,
    ) -> Result<DataStructure<Key, DataType, LinkType>, errors::DBOperationError>
    where
        Key: std::clone::Clone + std::cmp::Ord + Serialize,
        DataType: std::clone::Clone + Serialize,
        LinkType: std::clone::Clone + Serialize,
    {
        Err(errors::DBOperationError::NodeNotLink)
    }
    ///
    /// ```
    ///  let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    ///   let s = ds.to_string();
    /// ```
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
    /// ```
    /// std::fs::remove_file("db.json");
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// ds.insert(1,1);
    /// ds.make_backed("db.json".to_string());
    /// ds.quit();
    /// let mut ds2 = gulkana::ServiceController::<u32,u32,u32>::backed("db.json".to_string()).ok().unwrap();
    /// assert_eq!(ds2.get(1).ok().unwrap(),1);
    ///
    /// ```
    pub fn make_backed(&mut self, file_backing: String) -> Result<(), errors::DBOperationError> {
        match self.send_command(Command::MakeBacked(file_backing)) {
            CommandResult::InsertOk => Ok(()),
            CommandResult::Error(e) => Err(e),
            _ => Err(errors::DBOperationError::Other),
        }
    }
    /// Gets number of elements in db
    /// ```
    /// let mut ds = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// assert_eq!(ds.len().ok().unwrap(),0);
    /// ds.insert(20,20);
    /// assert_eq!(ds.len().ok().unwrap(),1);
    /// ```
    pub fn len(&mut self) -> Result<usize,errors::DBOperationError> {
        match self.send_command(Command::GetLen){
            CommandResult::GetLen(l)=>Ok(l),
            CommandResult::Error(e)=>Err(e),
            _ =>Err(errors::DBOperationError::Other),
        }
    }
}
///Holds Database and Access to Services
pub struct ServiceController<
    Key: 'static + std::marker::Sync + std::marker::Send + std::clone::Clone + Serialize + std::cmp::Ord,
    DataType: 'static + std::marker::Sync + std::marker::Send + std::clone::Clone + Serialize,
    LinkType: 'static + std::marker::Sync + std::marker::Send + std::clone::Clone + Serialize,
> {
    db: DataStructure<Key, DataType, LinkType>,
    service: Vec<ServiceDB<Key, DataType, LinkType>>,
}
impl<
        Key: 'static
            + std::marker::Sync
            + std::marker::Send
            + std::clone::Clone
            + Serialize
            + std::cmp::Ord
            + DeserializeOwned,
        DataType: 'static
            + std::marker::Sync
            + std::marker::Send
            + std::clone::Clone
            + Serialize
            + DeserializeOwned,
        LinkType: 'static
            + std::marker::Sync
            + std::marker::Send
            + std::clone::Clone
            + Serialize
            + DeserializeOwned
            + std::cmp::PartialEq,
    > ServiceController<Key, DataType, LinkType>
{
    pub fn backed(
        path: String,
    ) -> Result<ServiceClient<Key, DataType, LinkType>, crate::errors::DBOperationError> {
        let c: fn(String) -> ServiceController<Key, DataType, LinkType> =
            |path| ServiceController {
                db: backed_datastructure(&path).ok().unwrap(),
                service: vec![],
            };
        Ok(Self::make_controller_thread(c, path))
    }
    pub fn empty() -> ServiceClient<Key, DataType, LinkType> {
        Self::make_controller_thread(
            |()| ServiceController {
                db: new_datastructure(),
                service: vec![],
            },
            (),
        )
    }
    /// The main thread of the program
    fn main_loop(&mut self) {
        loop {
            let mut quit = false;
            let mut command_vec = vec![];
            command_vec.reserve(self.service.len());
            for mut service in &mut self.service {
                command_vec.push(service.get_command());
            }
            let mut i = 0;
            let mut res_vec = vec![];
            for command in command_vec {
                if command.is_some() {
                    let res = self.process_task(command.unwrap());
                    let r = match res {
                        CommandResult::Quit => true,
                        _ => false,
                    };
                    if r == true {
                        quit = true;
                    }
                    res_vec.push((i, res))
                }
                i += 1;
            }
            for (i, res) in res_vec {
                self.service[i].send_command_result(res);
            }
            if quit {
                break;
            }
        }
    }
    fn process_task(
        &mut self,
        command: Command<Key, DataType, LinkType>,
    ) -> CommandResult<Key, DataType, LinkType> {
        match command {
            Command::Quit => CommandResult::Quit,
            Command::Insert(key, data) => self.insert(key, data),
            Command::GetKeys(key) => self.get(key),
            Command::GetAllData => self.getData(),
            Command::GetLinkTypeNOT_USED(_link) => CommandResult::InsertOk,
            Command::GetContains(key) => self.getContains(key),
            Command::InsertLink(key, children, link_type) => {
                self.insert_link(key, children, link_type)
            }
            Command::GetLinkedData(key) => self.get_linked_data(key),
            Command::OverwriteData(key, data) => self.overwrite_data(key, data),
            Command::OverwriteLink(key, linked_keys, link_type) => {
                self.overwrite_link(key, linked_keys, link_type)
            }
            Command::GetLinkedKeys(key) => self.get_linked_keys(key),
            Command::GetAllKeys => self.get_all_keys(),
            Command::IterLinkType(l) => self.iter_link_type(l),
            Command::MakeBacked(s) => self.make_backed(s),
            Command::GetLen=>self.get_len(),
        }
    }
    fn get_len(&self)->CommandResult<Key, DataType, LinkType>{
        CommandResult::GetLen(self.db.len())
    }
    fn make_backed(&mut self, backing: String) -> CommandResult<Key, DataType, LinkType> {
        let r = self.db.make_backed(&backing);
        match r {
            Ok(_) => CommandResult::InsertOk,
            Err(e) => CommandResult::Error(e),
            _ => CommandResult::Error(errors::DBOperationError::Other),
        }
    }
    fn iter_link_type(&mut self, link: LinkType) -> CommandResult<Key, DataType, LinkType> {
        CommandResult::IterLinkType(
            self.db
                .iter_link_type(&link)
                .map(|(k, v)| (k, v.clone()))
                .collect::<Vec<(Key, Vec<Key>)>>()
                .clone(),
        )
    }
    fn get_all_keys(&mut self) -> CommandResult<Key, DataType, LinkType> {
        CommandResult::GetAllKeys(self.db.get_keys())
    }
    fn get_linked_keys(&mut self, key: Key) -> CommandResult<Key, DataType, LinkType> {
        match self.db.get_links(&key) {
            Ok(data) => CommandResult::GetLinkedKeys(data.clone()),
            Err(e) => CommandResult::Error(e),
        }
    }
    fn overwrite_link(
        &mut self,
        key: Key,
        linked_keys: Vec<Key>,
        link_type: LinkType,
    ) -> CommandResult<Key, DataType, LinkType> {
        match self.db.overwrite_link(&key, &linked_keys, link_type) {
            Ok(_) => CommandResult::InsertOk,
            Err(e) => CommandResult::Error(e),
        }
    }
    fn overwrite_data(
        &mut self,
        key: Key,
        data: DataType,
    ) -> CommandResult<Key, DataType, LinkType> {
        match self.db.set_data(&key, &data) {
            Ok(_i) => CommandResult::InsertOk,
            Err(e) => CommandResult::Error(e),
        }
    }
    fn get_linked_data(&self, key: Key) -> CommandResult<Key, DataType, LinkType> {
        let data = self.db.get_links(&key);
        if data.is_ok() {
            let d = data
                .ok()
                .unwrap()
                .iter()
                .map(|temp_key| {
                    (
                        temp_key.clone(),
                        self.db.get(&temp_key).ok().unwrap().clone(),
                    )
                })
                .collect();
            CommandResult::GetLinkedData(d)
        } else {
            return CommandResult::Error(errors::DBOperationError::Other);
        }
    }
    fn insert_link(
        &mut self,
        key: Key,
        children: Vec<Key>,
        link_type: LinkType,
    ) -> CommandResult<Key, DataType, LinkType> {
        match self.db.insert_link(&key, &children, link_type) {
            Ok(data) => CommandResult::InsertOk,
            Err(data) => CommandResult::Error(data),
        }
    }
    fn getData(&self) -> CommandResult<Key, DataType, LinkType> {
        CommandResult::ReturnAllData(
            self.db
                .iter_data()
                .map(|(a, b)| (a.clone(), b.clone()))
                .collect(),
        )
    }
    fn getContains(&self, key: Key) -> CommandResult<Key, DataType, LinkType> {
        CommandResult::Contains(self.db.contains(&key))
    }
    fn make_controller_thread<Args: 'static + std::marker::Send>(
        s: fn(Args) -> ServiceController<Key, DataType, LinkType>,
        args: Args,
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
    fn insert(&mut self, key: Key, data: DataType) -> CommandResult<Key, DataType, LinkType> {
        let res = self.db.insert(&key, data);
        if res.is_ok() {
            return CommandResult::InsertOk;
        } else {
            return CommandResult::Error(res.err().unwrap());
        }
    }
    fn get(&mut self, key: Key) -> CommandResult<Key, DataType, LinkType> {
        let res = self.db.get(&key);
        if res.is_ok() {
            return CommandResult::Get(res.ok().unwrap().clone());
        } else {
            return CommandResult::Error(res.err().unwrap());
        }
    }
}
fn new_client<
    Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq,
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
    fn quit_service_controller() {
        let mut c = ServiceController::<u32, u32, u32>::empty();
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
    #[test]
    fn backed() {
        let db_path = "ds.json".to_string();
        std::fs::remove_file(db_path.clone());
        {
            let mut ds = ServiceController::<u32, u32, u32>::empty();
            ds.insert(1, 1);
            ds.make_backed(db_path.clone());
            ds.quit();
        }
        let mut ds2 = ServiceController::<u32, u32, u32>::backed(db_path)
            .ok()
            .unwrap();
        let r = ds2.get(1);
        assert!(r.is_ok());
        assert_eq!(ds2.get(1).ok().unwrap(), 1);
    }
}
