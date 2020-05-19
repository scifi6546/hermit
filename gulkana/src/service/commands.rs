use crate::errors;
/// Used to send commands in between the Client and Master databases
#[derive(std::fmt::Debug, std::cmp::PartialEq)]
pub enum Command<Key: std::marker::Send, DataType: std::marker::Send, LinkType: std::marker::Send> {
    GetKeys(Key),
    Insert(Key, DataType),
    GetLinkTypeNOT_USED(LinkType),
    GetAllData,
    GetContains(Key),
    InsertLink(Key,Vec<Key>,LinkType),
    GetLinkedData(Key),
    OverwriteData(Key,DataType),
    //Used to send Quit service to database
    Quit,
}
pub enum CommandResult<
    Key: std::marker::Sync + std::marker::Send,
    DataType: std::marker::Sync + std::marker::Send,
    LinkType: std::marker::Sync + std::marker::Send,
> {
    InsertOk,
    Get(DataType),
    Quit,
    Error(errors::DBOperationError),
    ReturnAllData(Vec<(Key,DataType)>),
    Contains(bool),
    GetLinkedData(Vec<(Key,DataType)>),
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