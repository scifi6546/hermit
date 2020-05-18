pub struct DataIter<KeyType,DataType>{
    data: Vec<(KeyType,DataType)>,
    current_index:usize,
}
impl<KeyType,DataType> DataIter<KeyType,DataType>{
    pub fn new(data:Vec<(KeyType,DataType)>)->Self{
        Self{
            data:data,
            current_index:0,
        }
    }
}
impl<KeyType:std::clone::Clone,DataType:std::clone::Clone> Iterator for DataIter<KeyType,DataType>{
    type Item = (KeyType,DataType);
    fn next(&mut self)->Option<Self::Item>{
        if self.current_index<self.data.len(){
            let item = self.data[self.current_index].clone();
            self.current_index+=1;
            return Some(item);
        }else{
            None
        }

    }
}