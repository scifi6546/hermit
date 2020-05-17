use crate::{backed_datastructure, new_datastructure, DataStructure};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::errors;
///Service for use Database Side
pub struct ServiceDB<Key, DataType, LinkType> {
    send_result: Sender<CommandResult<Key, DataType, LinkType>>,
    recieve_commands: Receiver<Command<Key, DataType, LinkType>>,
}
impl<Key, DataType, LinkType> ServiceDB<Key, DataType, LinkType> {
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
pub struct ServiceClient<Key, DataType, LinkType> {
    send_commands: Sender<Command<Key, DataType, LinkType>>,
    recieve_result: Receiver<CommandResult<Key, DataType, LinkType>>,
}
impl<Key, DataType, LinkType> ServiceClient<Key, DataType, LinkType> {
    fn send_command(&mut self, command: Command<Key, DataType, LinkType>) {
        self.send_commands.send(command);
    }
    /// Inserts data into datastructure
    /// ```
    /// let mut ds = gulkana::service::ServiceController::empty::<u32,u32,u32>();
    /// let mut s = ds.add_service()
    /// let t = thread::spawn(||move{
    ///     s.insert(0,0); 
    ///  
    /// })
    /// t.join();
    /// ```
    pub fn insert(&mut self, key: &Key, data: DataType) -> Result<(), errors::DBOperationError>{
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
    pub fn get(&self, key: &Key) -> Result<&DataType, errors::DBOperationError> {
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
    pub fn iter_links(
        &self,
        key: &Key,
    ) -> Result<(), errors::DBOperationError> {
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
    pub fn iter_link_type(
        &self,
        link_type: &LinkType,
    ) ->()
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
        Key:std::clone::Clone+std::cmp::Ord+Serialize,
        DataType:std::clone::Clone+Serialize,
        LinkType:std::clone::Clone+Serialize,
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
enum Command<Key, DataType, LinkType> {
    GetKeys(Key),
    InsertNode(Key, DataType),
    GetLinkType(LinkType),
}
struct CommandResult<Key, DataType, LinkType> {
    key: Option<Key>,
    data: Option<DataType>,
    link: Option<LinkType>,
}
///Holds Database and Access to Services
struct ServiceController<
    Key: std::clone::Clone + std::cmp::Ord + Serialize,
    DataType: std::clone::Clone + std::cmp::Ord + Serialize,
    LinkType: std::clone::Clone + std::cmp::Ord + Serialize,
> {
    db: DataStructure<Key, DataType, LinkType>,
    service: Vec<ServiceDB<Key, DataType, LinkType>>,
}
impl<
        Key: std::clone::Clone + std::cmp::Ord + Serialize + DeserializeOwned,
        DataType: std::clone::Clone + std::cmp::Ord + Serialize + DeserializeOwned,
        LinkType: std::clone::Clone + std::cmp::Ord + Serialize + DeserializeOwned,
    > ServiceController<Key, DataType, LinkType>
{
    pub fn backed(
        path: &String,
    ) -> Result<ServiceController<Key, DataType, LinkType>, crate::errors::DBOperationError> {
        Ok(ServiceController {
            db: backed_datastructure(path)?,
            service: vec![],
        })
    }
    pub fn empty() -> ServiceController<Key, DataType, LinkType> {
        ServiceController {
            db: new_datastructure(),
            service: vec![],
        }
    }
    pub fn add_service(&mut self) -> ServiceClient<Key, DataType, LinkType> {
        let (db, client) = new_client();
        self.service.push(db);
        return client;
    }
}

pub fn new_client<Key, DataType, LinkType>() -> (
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
}
