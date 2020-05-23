use crate::errors;
use crate::service::ServiceClient;
use serde::Serialize;
pub struct JoinFn<
Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq+std::clone::Clone+Serialize+std::cmp::Ord+std::fmt::Display,
DataType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
LinkType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
> {
    pub function:fn(&mut ServiceClient<Key,DataType,LinkType>)->Result<(),errors::DBOperationError>
}
impl<
Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq+std::clone::Clone+Serialize+std::cmp::Ord+std::fmt::Display,
DataType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
LinkType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
>   PartialEq for JoinFn<Key,DataType,LinkType>{
    fn eq(&self,other:&Self)->bool{
        false
    }
}
impl<
Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq+std::clone::Clone+Serialize+std::cmp::Ord+std::fmt::Display,
DataType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
LinkType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
> std::fmt::Debug for JoinFn<Key,DataType,LinkType>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JoinFn")
         .finish()
    }
}
/// Used to send commands in between the Client and Master databases
#[derive( std::cmp::PartialEq,std::fmt::Debug)]
pub enum Command<
Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq+std::clone::Clone+Serialize+std::cmp::Ord+std::fmt::Display,
DataType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
LinkType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
>  {
    GetKeys(Key),
    Insert(Key, DataType),
    GetLinkTypeNOT_USED(LinkType),
    GetAllData,
    GetContains(Key),
    InsertLink(Key,Vec<Key>,LinkType),
    GetLinkedData(Key),
    OverwriteData(Key,DataType),
    OverwriteLink(Key,Vec<Key>,LinkType),
    GetLinkedKeys(Key),
    GetAllKeys,
    IterLinkType(LinkType),
    MakeBacked(String),
    GetLen,
    RightJoin(JoinFn<Key,DataType,LinkType>),
    GetDB,
    AppendLink(Key,Key),
    //Used to send Quit service to database
    Quit,
}
pub enum CommandResult<
Key: std::marker::Sync + std::marker::Send+std::cmp::PartialEq+std::clone::Clone+Serialize+std::cmp::Ord+std::fmt::Display,
DataType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
LinkType: std::marker::Sync + std::marker::Send+std::clone::Clone+Serialize,
> {
    InsertOk,
    Get(DataType),
    Quit,
    Error(errors::DBOperationError),
    ReturnAllData(Vec<(Key,DataType)>),
    Contains(bool),
    GetLinkedData(Vec<(Key,DataType)>),
    GetLinkedKeys(Vec<Key>),
    GetAllKeys(Vec<Key>),
    IterLinkType(Vec<(Key,Vec<Key>)>),
    GetLen(usize),
    GetDB(crate::DataStructure<Key,DataType,LinkType>),
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