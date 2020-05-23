use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::clone::Clone;
use rand::prelude;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::errors::*;
mod node;
use node::*;
/// # Gulkana
/// Gulkana is a lightweight key based database for string files.
/// The main struct is DataStructure

/// Struct usd to store data
/// Inorder to allow new fields in input struct to be added
/// make all fields Optional e.g.
/// ```
/// struct bar{
///     foo:Option<String>,
///     bar:Option<u32>,
/// }
/// ```
/// this way the data structure is compatible with old versions of the database.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DataStructure<
    KeyType: std::cmp::Ord + std::clone::Clone + Serialize,
    DataType: std::clone::Clone + Serialize,
    LinkLabel: std::clone::Clone + Serialize,
> {
    tree: BTreeMap<KeyType, Node<KeyType, DataType, LinkLabel>>,
    file_backing: Option<String>, // file to write back to
}
///Iterator over all data nodes
pub struct DataNodeIter<
    'a,
    KeyType: std::cmp::Ord + std::clone::Clone,
    DataType: std::clone::Clone,
    LinkLabel: std::clone::Clone,
> {
    iter: std::collections::btree_map::Iter<'a, KeyType, Node<KeyType, DataType, LinkLabel>>,
}
impl<
        'a,
        KeyType: std::cmp::Ord + std::clone::Clone,
        DataType: std::clone::Clone,
        LinkLabel: std::clone::Clone,
    > Iterator for DataNodeIter<'a, KeyType, DataType, LinkLabel>
{
    type Item = (&'a KeyType, &'a DataType);
    fn next(&mut self) -> Option<Self::Item> {
        let data = self.iter.next();
        if data.is_none() {
            return None;
        } else {
            let (key, node_unwrapped) = data.unwrap();
            //getting data in node opt_pair;
            let data_opt = node_unwrapped.item.b();
            if data_opt.is_none() {
                return self.next();
            } else {
                return Some((key, data_opt.unwrap()));
            }
        }
    }
}
pub struct DataMutIter<
    'a,
    KeyType: std::cmp::Ord + std::clone::Clone,
    DataType: std::clone::Clone,
    LinkLabel: std::clone::Clone,
> {
    iter: std::collections::btree_map::IterMut<'a, KeyType, Node<KeyType, DataType, LinkLabel>>,
}
impl<
        'a,
        KeyType: std::cmp::Ord + std::clone::Clone,
        DataType: std::clone::Clone,
        LinkLabel: std::clone::Clone,
    > Iterator for DataMutIter<'a, KeyType, DataType, LinkLabel>
{
    type Item = (&'a KeyType, &'a mut DataType);
    fn next(&mut self) -> Option<Self::Item> {
        let data = self.iter.next();
        if data.is_none() {
            return None;
        } else {
            let (key, node_unwrapped) = data.unwrap();
            //getting data in node opt_pair;
            let data_opt = node_unwrapped.item.b_mut();
            if data_opt.is_none() {
                return self.next();
            } else {
                return Some((key, data_opt.unwrap()));
            }
        }
    }
}
pub struct DataLinkIter<
    'a,
    KeyType: std::cmp::Ord + std::clone::Clone + Serialize,
    DataType: std::clone::Clone + Serialize,
    LinkLabel: std::clone::Clone + Serialize,
> {
    db: &'a DataStructure<KeyType, DataType, LinkLabel>,
    linked_keys: &'a std::vec::Vec<KeyType>,
    current_index: usize,
}
impl<
        'a,
        KeyType: std::cmp::Ord + std::clone::Clone + Serialize,
        DataType: std::clone::Clone + Serialize,
        LinkLabel: std::clone::Clone + Serialize,
    > Iterator for DataLinkIter<'a, KeyType, DataType, LinkLabel>
{
    type Item = (&'a KeyType, &'a DataType);
    fn next(&mut self) -> Option<Self::Item> {
        let opt = self.linked_keys.get(self.current_index);
        if opt.is_some() {
            let res = self.db.get(&opt.unwrap().clone());
            if res.is_ok() {
                let data = res.ok().unwrap();
                self.current_index += 1;
                return Some((&opt.unwrap(), data));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
pub struct DataLinkIterNoRef<
    'a,
    KeyType: std::cmp::Ord + std::clone::Clone + Serialize,
    DataType: std::clone::Clone + Serialize,
    LinkLabel: std::clone::Clone + Serialize,
> {
    db: &'a DataStructure<KeyType, DataType, LinkLabel>,
    linked_keys: std::vec::Vec<KeyType>,
    current_index: usize,
}
impl<
        'a,
        KeyType: std::cmp::Ord + std::clone::Clone + Serialize,
        DataType: std::clone::Clone + Serialize,
        LinkLabel: std::clone::Clone + Serialize,
    > Iterator for DataLinkIterNoRef<'a, KeyType, DataType, LinkLabel>
{
    type Item = (KeyType, &'a std::vec::Vec<KeyType>);
    fn next(&mut self) -> Option<Self::Item> {
        let opt = self.linked_keys.get(self.current_index);
        if opt.is_some() {
            let res = self.db.get_links(&opt.unwrap().clone());
            if res.is_ok() {
                let data = res.ok().unwrap();
                self.current_index += 1;
                return Some((opt.unwrap().clone(), data));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
impl<
        KeyType: std::cmp::Ord + std::clone::Clone + Serialize,
        DataType: std::clone::Clone + Serialize,
        LinkLabel: std::clone::Clone + Serialize + Serialize,
    > DataStructure<KeyType, DataType, LinkLabel>
{
    /// Inserts data into datastructure
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// assert!(ds.insert(&10,20).is_err());
    /// ```
    pub fn insert(&mut self, key: &KeyType, data: DataType) -> Result<(), DBOperationError> {
        return self.insert_node(key, new_node(data));
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
        key: &KeyType,
        children: &std::vec::Vec<KeyType>,
        link_type: LinkLabel,
    ) -> Result<(), DBOperationError> {
        return self.insert_node(key, new_node_link(children, link_type));
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
        key: &KeyType,
        children: &std::vec::Vec<KeyType>,
        link_type: LinkLabel,
    ) -> Result<(), DBOperationError> {
        return self.overwrite_node(key, new_node_link(children, link_type));
    }

    fn insert_node(
        &mut self,
        key: &KeyType,
        data: Node<KeyType, DataType, LinkLabel>,
    ) -> Result<(), DBOperationError> {
        if self.tree.contains_key(key) == false {
            self.tree.insert(key.clone(), data);
            self.write_back()?;
            return Ok(());
        } else {
            return Err(DBOperationError::KeyAllreadyPresent);
        }
    }
    fn overwrite_node(
        &mut self,
        key: &KeyType,
        data: Node<KeyType, DataType, LinkLabel>,
    ) -> Result<(), DBOperationError> {
        self.tree.insert(key.clone(), data);
        self.write_back()?;
        return Ok(());
    }
    /// sets data in database
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,3);
    /// ds.set_data(&10,&5);
    /// assert!(ds.get(&10).ok().unwrap()==&5);
    /// ```
    pub fn set_data(&mut self, key: &KeyType, data: &DataType) -> Result<(), DBOperationError> {
        self.overwrite_node(key, new_node(data.clone()))
    }
    fn iter(
        &self,
    ) -> std::collections::btree_map::Iter<'_, KeyType, Node<KeyType, DataType, LinkLabel>> {
        self.tree.iter()
    }
    /// Used to iterate through data
    /// ```
    ///  let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    ///  ds.insert(&10,3);
    ///  for (key,data) in ds.iter_data(){
    ///     assert_eq!(key,&10);
    ///     assert_eq!(data,&3);
    /// }
    /// ````
    pub fn iter_data(&self) -> DataNodeIter<KeyType, DataType, LinkLabel> {
        return DataNodeIter { iter: self.iter() };
    }
    /// Gets All keys in database
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// let out = ds.get_keys();
    /// assert!(out[0]==10);
    /// ```
    pub fn get_keys(&self) -> std::vec::Vec<KeyType> {
        return self.tree.keys().cloned().collect();
    }
    /// gets key from database
    /// ```
    ///
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// let data = ds.get(&10);
    /// assert!(*data.ok().unwrap()==5);
    /// ```
    pub fn get(&self, key: &KeyType) -> Result<&DataType, DBOperationError> {
        let temp = self.tree.get(key);
        if temp.is_none() {
            return Err(DBOperationError::KeyNotFound);
        } else {
            return temp.unwrap().get_item();
        }
    }
    fn get_node(
        &self,
        key: &KeyType,
    ) -> Result<&Node<KeyType, DataType, LinkLabel>, DBOperationError> {
        let item = self.tree.get(key);
        if item.is_some() {
            return Ok(item.unwrap());
        } else {
            return Err(DBOperationError::KeyNotFound);
        }
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

    pub fn get_links(&self, key: &KeyType) -> Result<&Vec<KeyType>, DBOperationError> {
        let data = self.get_node(key)?;
        let vec_temp = data.item.a();
        if vec_temp.is_some() {
            return Ok(&vec_temp.unwrap().children);
        } else {
            return Err(DBOperationError::NodeNotLink);
        }
    }
    /// Iterates through nodes attached to link
    ///
    pub fn iter_links(
        &self,
        key: &KeyType,
    ) -> Result<DataLinkIter<KeyType, DataType, LinkLabel>, DBOperationError> {
        return Ok(DataLinkIter {
            db: self,
            linked_keys: self.get_links(key)?,
            current_index: 0,
        });
    }
    /// Checks if database contains a given key
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// assert!(ds.contains(&10));
    /// assert!(!ds.contains(&20));
    /// ```
    pub fn contains(&self, key: &KeyType) -> bool {
        return self.tree.get(key).is_some();
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
        link_type: &LinkLabel,
    ) -> DataLinkIterNoRef<KeyType, DataType, LinkLabel>
    where
        LinkLabel: std::cmp::PartialEq,
    {
        let mut keys = vec![];

        for (key, node) in self.iter() {
            let res = node.get_link();
            if res.is_ok() {
                if &res.ok().unwrap().type_label == link_type {
                    keys.push(key.clone());
                }
            }
        }
        return DataLinkIterNoRef {
            db: self,
            linked_keys: keys,
            current_index: 0,
        };
    }
    pub fn append_links(
        &mut self,
        key: &KeyType,
        key_append: &KeyType,
    ) -> Result<(), DBOperationError> {
        let data = self.get_node(key)?.clone();
        let link_vec_opt = data.item.a();
        if link_vec_opt.is_some() {
            let link = link_vec_opt.unwrap();
            let mut link_vec = link.children.clone();
            if !link_vec.contains(key_append) {
                link_vec.push(key_append.clone());
                return self.overwrite_link(key, &link_vec, link.type_label.clone());
            } else {
                return Err(DBOperationError::KeyAllreadyPresent);
            }
        } else {
            return Err(DBOperationError::NodeNotLink);
        }
    }
    pub fn right_join(
        &self,
        right: &DataStructure<KeyType, DataType, LinkLabel>,
    ) -> Result<DataStructure<KeyType, DataType, LinkLabel>, DBOperationError> {
        return right_join(self, right);
    }
    pub fn to_string(&self) -> Result<std::string::String, SerializeError>
    where
        KeyType: Serialize,
        DataType: Serialize,
        LinkLabel: Serialize,
    {
        let res = serde_json::to_string(&self);
        if res.is_ok() {
            return Ok(res.ok().unwrap());
        } else {
            match res.err().unwrap() {
                _ => return Err(SerializeError::Unknown),
            }
        }
    }
    /// Makes the database backed
    ///
    pub fn make_backed(&mut self, file_backing: &String) -> Result<(), DBOperationError> {
        self.file_backing = Some(file_backing.clone());
        self.write_back()?;
        Ok(())
    }
    ///writes back to a file
    fn write_back(&mut self) -> Result<(), DBOperationError>
    where
        KeyType: Serialize,
        DataType: Serialize,
        LinkLabel: Serialize,
    {
        if self.file_backing.is_some() {
            let mut file = File::create(self.file_backing.clone().unwrap())?;
            let out_str = self.to_string()?;

            file.write_all(out_str.as_bytes())?;

            return Ok(());
        } else {
            return Ok(());
        }
    }
    /// Gets number of elements in db
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// assert!(ds.len()==0);
    /// ds.insert(&20,20);
    /// assert!(ds.len()==1);
    /// ```
    pub fn len(&self) -> usize {
        return self.tree.len();
    }
}
impl<
        K: std::cmp::Ord + std::fmt::Display + std::clone::Clone + Serialize,
        DataType: std::clone::Clone + Serialize,
        I: std::clone::Clone + Serialize,
    > fmt::Display for DataStructure<K, DataType, I>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for row in self.iter() {
            write!(f, "\tkey: {}\n", row.0)?;
        }
        return write!(f, "");
    }
}
impl<
K: std::cmp::Ord + std::fmt::Display + std::clone::Clone + Serialize,
D: std::clone::Clone + Serialize,
I: std::clone::Clone + Serialize,
>  std::fmt::Debug for DataStructure<K,D,I>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JoinFn")
        .field("data", &format!("{}",self))
         .finish()
    }
}
pub enum ReadError {
    ParseError,
}
pub fn right_join<
    K: std::cmp::Ord + std::clone::Clone + Serialize,
    DataType: std::clone::Clone + Serialize,
    LinkLabel: std::clone::Clone + Serialize,
>(
    left: &DataStructure<K, DataType, LinkLabel>,
    right: &DataStructure<K, DataType, LinkLabel>,
) -> Result<DataStructure<K, DataType, LinkLabel>, DBOperationError> {
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    let mut db = new_datastructure::<K, DataType, LinkLabel>();

    loop {
        let left_opt = left_iter.peek();
        let right_opt = right_iter.peek();
        if right_opt.is_none() {
            return Ok(db);
        } else {
            if left_opt.is_none() {
                db.insert_node(right_opt.unwrap().0, right_opt.unwrap().1.clone())?;
                right_iter.next();
            } else {
                let left_data = left_opt.unwrap();
                let right_data = right_opt.unwrap();
                let left_key = left_data.0;
                let right_key = right_data.0;
                //if keys are the same
                if left_key == right_key {
                    db.insert_node(left_key, left_data.1.clone())?;
                    left_iter.next();
                    right_iter.next();
                } else if left_key < right_key {
                    left_iter.next();
                } else if left_key > right_key {
                    db.insert_node(right_key, right_data.1.clone())?;
                    right_iter.next();
                }
            }
        }
    }
}
pub fn new_datastructure<
    K: std::cmp::PartialEq + std::clone::Clone + std::cmp::Ord + Serialize,
    DataType: std::clone::Clone + Serialize,
    LinkLabel: std::clone::Clone + Serialize,
>() -> DataStructure<K, DataType, LinkLabel> {
    return DataStructure {
        tree: BTreeMap::new(),
        file_backing: None,
    };
}
pub fn from_string<
    K: std::cmp::PartialEq + std::clone::Clone + std::cmp::Ord + Serialize,
    DataType: std::clone::Clone + Serialize,
    LinkLabel: std::clone::Clone + Serialize,
>(
    input_string: String,
) -> Result<DataStructure<K, DataType, LinkLabel>, DBOperationError>
where
    K: DeserializeOwned,

    DataType: DeserializeOwned,
    LinkLabel: DeserializeOwned,
{
    let ds = serde_json::from_str(input_string.as_str())?;
    return Ok(ds);
}
pub fn backed_datastructure<
    K: std::cmp::PartialEq + std::clone::Clone + std::cmp::Ord + Serialize,
    DataType: std::clone::Clone + Serialize,
    LinkLabel: std::clone::Clone + Serialize,
>(
    backing: &String,
) -> Result<DataStructure<K, DataType, LinkLabel>, DBOperationError>
where
    K: DeserializeOwned,

    DataType: DeserializeOwned,
    LinkLabel: DeserializeOwned,
{
    let file_res = File::open(backing);

    if file_res.is_ok() {
        let file = file_res.ok().unwrap();
        let len = file.metadata().unwrap().len();
        let res = serde_json::from_reader(file);
        if res.is_ok() {
            return Ok(res.ok().unwrap());
        } else {
            if len < 20 {
                let mut ds = DataStructure {
                    tree: BTreeMap::new(),
                    file_backing: Some(backing.clone()),
                };
                ds.write_back()?;
                return Ok(ds);
            }
            return Err(DBOperationError::ParseError);
        }
    } else {
        let mut ds = DataStructure {
            tree: BTreeMap::new(),
            file_backing: Some(backing.clone()),
        };
        ds.write_back()?;
        return Ok(ds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Label = u32;

    #[test]
    #[allow(unused_must_use)]
    fn test_insert() {
        let mut arr: Vec<u32> = Vec::new();
        arr.reserve(100000);
        for _i in 1..100000 {
            arr.push(_i);
        }

        let mut ds = new_datastructure::<u32, u32, Label>();
        for i in &arr {
            ds.insert(i, *i);
        }
        let mut test_arr: Vec<u32> = Vec::new();
        for (_key, data) in ds.iter() {
            test_arr.push(*data.item.b().unwrap());
        }
        arr.sort();
        test_arr.sort();
        for i in 0..test_arr.len() {
            assert!(arr[i] == test_arr[i]);
        }
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_join_perf() {
        let mut arr: Vec<u32> = Vec::new();
        arr.reserve(100000);
        for _i in 1..100000 {
            arr.push(_i);
        }

        let mut ds = new_datastructure::<u32, u32, Label>();
        for i in &arr {
            ds.insert(i, *i);
        }
        let mut dsr = new_datastructure::<u32, u32, Label>();
        for i in &arr {
            dsr.insert(i, *i);
        }
        let dsj = right_join(&ds, &dsr).ok().unwrap();
        arr.sort();
        let mut test_arr: Vec<u32> = Vec::new();
        for (_key, data) in dsj.iter() {
            test_arr.push(*data.item.b().unwrap());
        }
    }
    #[test]
    fn write_back_join() {}
    #[test]
    #[allow(unused_must_use)]
    fn test_backed() {
        let db_path = "test_backed.json";
        {
            std::fs::remove_file(db_path);
            let mut ds = backed_datastructure::<u32, u32, Label>(&db_path.to_string())
                .ok()
                .unwrap();
            ds.insert(&0, 0);
        }
        {
            let ds = backed_datastructure::<u32, u32, Label>(&db_path.to_string())
                .ok()
                .unwrap();
            let data = ds.get(&0).ok().unwrap();
            assert!(data == &0);
            std::fs::remove_file(db_path);
        }
    }
    #[test]
    #[allow(unused_must_use, unused_variables)]
    fn test_make_backed() {
        let db_path = "test_make_backed.json";
        {
            std::fs::remove_file(db_path);
            let mut ds = new_datastructure::<u32, u32, Label>();
            ds.insert(&0, 0);
            ds.make_backed(&db_path.to_string());
        }
        assert!(std::path::Path::new(db_path).exists());
        {
            let ds = backed_datastructure::<u32, u32, Label>(&db_path.to_string())
                .ok()
                .unwrap();
            let data = ds.get(&0).ok().unwrap();
            assert!(data == &0);
        }
        assert!(std::path::Path::new(db_path).exists());
        {
            std::fs::remove_file(db_path);
            let ds = backed_datastructure::<u32, u32, Label>(&db_path.to_string())
                .ok()
                .unwrap();
            assert!(std::path::Path::new(db_path).exists());
        }
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_right_join() {
        let mut dsr = new_datastructure::<u32, u32, Label>();
        dsr.insert(&0, 0);
        dsr.insert(&1, 1);
        dsr.insert(&2, 2);
        let mut dsl = new_datastructure::<u32, u32, Label>();
        dsl.insert(&0, 0);
        dsl.insert(&1, 1);
        dsl.insert(&2, 2);
        let join = right_join(&dsr, &dsl).ok().unwrap();
        assert!(join == dsl);

        dsr.insert(&4, 3);
        let join2 = right_join(&dsl, &dsr).ok().unwrap();
        assert!(join2 != dsl);
        dsl.insert(&4, 3);
        assert!(join2 == dsl);
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_eq() {
        let mut dsr = new_datastructure::<u32, u32, Label>();
        dsr.insert(&0, 0);
        dsr.insert(&1, 1);
        dsr.insert(&2, 2);
        let mut dsl = new_datastructure::<u32, u32, Label>();
        dsl.insert(&0, 0);
        dsl.insert(&1, 1);
        dsl.insert(&2, 2);
        assert!(dsr == dsl);
        dsl.insert(&3, 3);
        assert!(dsr != dsl);
    }
    #[test]
    #[allow(unused_must_use)]
    fn make_backed_with_file() {
        std::fs::remove_file("testing_db.json");
        {
            let mut file = File::create("testing_db.json").ok().unwrap();
            file.write_all(b"");
        }
        let ds = backed_datastructure::<u32, u32, Label>(&"testing_db.json".to_string());
        assert!(ds.is_ok() == true);
    }
    #[test]
    #[allow(unused_must_use, unused_variables)]
    fn test_backed_join() {
        let mut dsr = new_datastructure::<u32, u32, Label>();
        dsr.insert(&0, 0);
        dsr.insert(&1, 1);
        dsr.insert(&2, 2);
        for i in 3..100 {
            dsr.insert(&i, 5);
        }

        let mut dsl = backed_datastructure::<u32, u32, Label>(&"join.json".to_string())
            .ok()
            .unwrap();
        dsl.insert(&0, 0);
        dsl.insert(&1, 1);
        dsl.insert(&2, 2);


        let join = right_join(&dsr, &dsl).ok().unwrap();

        let join2 = right_join(&dsl, &dsr).ok().unwrap();


        dsr.insert(&4, 3);


        dsl.insert(&4, 3);
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_serialize() {
        let mut dsr = new_datastructure::<u32, u32, Label>();
        dsr.insert(&0, 0);
        dsr.insert(&1, 1);
        dsr.insert(&2, 2);
        let str_ds = dsr.to_string();
        let dsl: DataStructure<u32, u32, Label> = from_string(str_ds.ok().unwrap()).ok().unwrap();
        assert!(dsr == dsl);
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_links() {
        let mut dsr = new_datastructure::<u32, u32, Label>();
        dsr.insert(&0, 0);
        dsr.insert(&1, 1);
        dsr.insert(&2, 2);
        dsr.insert_link(&4, &vec![0, 1], 0);
        let foo: std::vec::Vec<u32> = vec![0, 1];
        assert!(*dsr.get_links(&4).ok().unwrap() == (foo));
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_iter_link() {
        let mut ds = new_datastructure::<u32, u32, Label>();
        ds.insert(&10, 5);
        ds.insert_link(&9, &vec![10], 0);
        let iter = ds.iter_links(&9).ok().unwrap();
        for (_i, j) in iter {
            assert!(*j == 5);
        }
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_iter_data() {
        let mut ds = new_datastructure::<u32, u32, Label>();
        ds.insert(&10, 5);
        for (_key, data) in ds.iter_data() {
            assert!(*data == 5);
        }
        return ();
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_set_data() {
        let mut ds = new_datastructure::<u32, u32, Label>();
        ds.insert(&10, 5);
        ds.set_data(&10, &10);
        for (_key, data) in ds.iter_data() {
            assert!(*data == 10);
        }
        return ();
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_link_type_iter() {
        let mut ds = new_datastructure::<u32, u32, u32>();
        ds.insert(&10, 5);
        ds.insert_link(&9, &vec![10], 0);
        for (key, linked_keys) in ds.iter_link_type(&0) {
            assert!(key == 9);
            assert!(linked_keys[0] == 10);
        }
    }
    #[test]
    #[allow(unused_must_use)]
    fn print_datastructure() {
        let mut arr: Vec<u32> = Vec::new();
        arr.reserve(100000);
        for _i in 1..100000 {
            arr.push(_i);
        }

        let mut ds = new_datastructure::<u32, u32, Label>();
        for i in &arr {
            ds.insert(i, *i);
        }
        let _s = format!("{}", ds);
    }
    #[test]
    fn test_to_string_error() {
        let _s: String = DBOperationError::KeyAllreadyPresent.into();
    }
    #[test]
    fn print_errors() {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            DBOperationError::KeyAllreadyPresent,
            DBOperationError::KeyNotFound,
            DBOperationError::NodeNotLink,
            DBOperationError::NodeNotData,
            DBOperationError::SerializeError,
            DBOperationError::FSError,
            DBOperationError::ParseError,
            DBOperationError::FileNotFound,
            DBOperationError::FilePermissionDenied,
            DBOperationError::NetworkConnectionRefused,
            DBOperationError::NetworkConnectionReset,
            DBOperationError::NetworkNotConnected,
            DBOperationError::NetworkAddressInUse,
            DBOperationError::NetworkAddrNotAvailable,
            DBOperationError::BrokenPipe,
            DBOperationError::FileAlreadyExists,
            DBOperationError::WouldBlock,
            DBOperationError::InvalidInput,
            DBOperationError::InvalidData,
            DBOperationError::TimedOut,
            DBOperationError::Interrupted,
            DBOperationError::Other,
            DBOperationError::UnexpectedEof,
        );
    }
}
