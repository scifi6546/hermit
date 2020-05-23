use crate::errors::*;
mod opt_pair;
use opt_pair::*;
use serde::{Deserialize, Serialize};
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Link<Key: std::clone::Clone, TypeLabel: std::clone::Clone> {
    pub type_label: TypeLabel,
    pub children: Vec<Key>,
}
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Node<
    Key: std::cmp::PartialEq + std::clone::Clone,
    Item: std::clone::Clone,
    LinkLabel: std::clone::Clone,
> {
    pub item: OptStruct<Link<Key, LinkLabel>, Item>,
}
impl<
        KeyType: std::cmp::PartialEq + std::clone::Clone,
        DataType: std::clone::Clone,
        LinkLabel: std::clone::Clone,
    > Node<KeyType, DataType, LinkLabel>
{
    pub fn get_item(&self) -> Result<&DataType, DBOperationError> {
        let data = self.item.b();
        if data.is_some() {
            return Ok(data.unwrap());
        } else {
            return Err(DBOperationError::NodeNotData);
        }
    }
    #[allow(unused)]
    pub fn get_item_mut(&mut self) -> Result<&mut DataType, DBOperationError> {
        let data = self.item.b_mut();
        if data.is_some() {
            return Ok(data.unwrap());
        } else {
            return Err(DBOperationError::NodeNotData);
        }
    }
    pub fn get_link(&self) -> Result<&Link<KeyType, LinkLabel>, DBOperationError> {
        let data = self.item.a();
        if data.is_some() {
            return Ok(data.unwrap());
        } else {
            return Err(DBOperationError::NodeNotLink);
        }
    }
}
pub fn new_node<
    K: std::cmp::PartialEq + std::clone::Clone,
    I: std::clone::Clone,
    LinkLabel: std::clone::Clone,
>(
    input: I,
) -> Node<K, I, LinkLabel>
where
    K: std::clone::Clone,
    I: std::clone::Clone,
{
    let foo = Node {
        item: new_optstruct_b(input),
    };
    return foo;
}
pub fn new_node_link<
    K: std::cmp::PartialEq + std::clone::Clone,
    I: std::clone::Clone,
    LinkLabel: std::clone::Clone,
>(
    input: &std::vec::Vec<K>,
    link_type: LinkLabel,
) -> Node<K, I, LinkLabel>
where
    K: std::clone::Clone,
    I: std::clone::Clone,
{
    let foo = Node {
        item: new_optstruct_a(Link {
            children: input.clone(),
            type_label: link_type,
        }),
    };
    return foo;
}
