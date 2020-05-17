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
    async fn send_command(
        &mut self,
        command: Command<Key, DataType, LinkType>,
    ) -> CommandResult<Key, DataType, LinkType> {
        self.send_commands.send(command);
        return self.recieve_result.recv().ok().unwrap();
    }
    /// Inserts data into datastructure
    /// ```
    /// let mut s = gulkana::ServiceController::<u32,u32,u32>::empty();
    /// let mut c = s.add_service();
    /// let t = std::thread::spawn(move || {
    ///     c.insert(0,0);
    ///     
    /// });
    /// ```
    pub async fn insert(
        &mut self,
        key: Key,
        data: DataType,
    ) -> Result<(), errors::DBOperationError> {
        self.send_command(Command::Insert(key, data));
        Ok(())
    }
    ///Used to insert a link into a datastructure
    ///```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
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
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// ds.insert(&11,6);
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
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,3);
    /// ds.set_data(&10,&5);
    /// assert!(ds.get(&10).ok().unwrap()==&5);
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
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// let out = ds.get_keys();
    /// assert!(out[0]==10);
    pub fn get_keys(&self) -> std::vec::Vec<Key> {
        vec![]
    }
    /// gets key from database
    /// ```
    ///
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// let data = ds.get(&10);
    /// assert!(*data.ok().unwrap()==5);
    /// ```
    pub async fn get(&self, key: &Key) -> Result<&DataType, errors::DBOperationError> {
        Err(errors::DBOperationError::BrokenPipe)
    }
    /// Gets linked nodes
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// ds.insert(&11,6);
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
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// assert!(ds.contains(&10));
    /// assert!(!ds.contains(&20));
    /// ```
    pub fn contains(&self, key: &Key) -> bool {
        false
    }
    /// Gets iterator of links with labels
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
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
#[derive(std::fmt::Debug, std::cmp::PartialEq)]
enum Command<Key: std::marker::Send, DataType: std::marker::Send, LinkType: std::marker::Send> {
    GetKeys(Key),
    Insert(Key, DataType),
    GetLinkType(LinkType),
}
struct CommandResult<
    Key: std::marker::Sync,
    DataType: std::marker::Sync,
    LinkType: std::marker::Sync,
> {
    key: Option<Key>,
    data: Option<DataType>,
    link: Option<LinkType>,
}
///Holds Database and Access to Services
pub struct ServiceController<
    Key: std::marker::Sync + std::marker::Send + std::clone::Clone+Serialize+std::cmp::Ord,
    DataType: std::marker::Sync + std::marker::Send + std::clone::Clone+Serialize,
    LinkType: std::marker::Sync + std::marker::Send + std::clone::Clone+Serialize,
> {
    db: DataStructure<Key, DataType, LinkType>,
    service: Vec<ServiceDB<Key, DataType, LinkType>>,
}
impl<
Key: std::marker::Sync + std::marker::Send + std::clone::Clone+Serialize+std::cmp::Ord+DeserializeOwned,
DataType: std::marker::Sync + std::marker::Send + std::clone::Clone+Serialize+DeserializeOwned,
LinkType: std::marker::Sync + std::marker::Send + std::clone::Clone+Serialize+DeserializeOwned,
>  ServiceController<Key, DataType, LinkType>
{
    pub fn backed(
        path: &String,
    ) -> Result<ServiceClient<Key, DataType, LinkType>, crate::errors::DBOperationError> {
        Ok(Self::make_controller_thread(&ServiceController {
            db: backed_datastructure(path)?,
            service: vec![],
        }))
    }
    pub fn empty() -> ServiceClient<Key, DataType, LinkType> {
        Self::make_controller_thread(&ServiceController {
            db: new_datastructure(),
            service: vec![],
        })
    }
    /// The main thread of the program
    fn main_loop(&mut self) {
        loop {}
    }
    fn make_controller_thread(
        s: &ServiceController<Key, DataType, LinkType>,
    ) -> ServiceClient<Key, DataType, LinkType> {
        let c = s.add_service();
        std::thread::spawn(move || {
            s.main_loop();
        });
        return c;
    }
    pub fn add_service(&mut self) -> ServiceClient<Key, DataType, LinkType> {
        let (db, client) = new_client();
        self.service.push(db);
        return client;
    }
}
fn new_client<Key: std::marker::Sync, DataType: std::marker::Sync, LinkType: std::marker::Sync>(
) -> (
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
        let (mut db, mut c) = new_client::<u32, u32, u32>();
        let t = std::thread::spawn(move || {
            c.send_command(Command::GetKeys(0));
        });
        #[allow(unused_must_use)]
        let _r = t.join();
        assert_eq!(db.get_command().unwrap(), Command::GetKeys(0));
    }
    #[test]
    fn insert_and_get() {
        let (mut db, mut c) = new_client::<u32, u32, u32>();
        block_on(c.insert(0, 0));
        let r = block_on(c.get(&0));
        assert_eq!(r.ok().unwrap(), &0);
        block_on(c.insert(1, 1));
        let r = block_on(c.get(&1));
        assert_eq!(r.ok().unwrap(), &1);
    }
}
