pub struct DataIter<Item>{
    data: Vec<Item>,
    current_index:usize,
}
impl<Item> DataIter<Item>{
    pub fn new(data:Vec<Item>)->Self{
        Self{
            data:data,
            current_index:0,
        }
    }
}
impl<Item:std::clone::Clone> Iterator for DataIter<Item>{
    type Item = Item;
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