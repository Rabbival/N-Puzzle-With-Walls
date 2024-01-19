use crate::prelude::*;

use crate::output::error_handler;
use std::hash::Hash;

pub struct LinkedList<T: Hash + Ord + Copy>{
	list: Vec<T>,
	index_by_value: HashMap<T, usize>,
	next_index: usize
}

impl<T: Hash + Ord + Copy> LinkedList<T>{
	pub fn new() -> LinkedList<T>{
		Self { 
			list: vec![],
			index_by_value: HashMap::new(),
			next_index: 0
		}
	}

	pub fn is_empty(&self) -> bool{
		self.next_index == 0
	}
	
	pub fn push(&mut self, next_value: T) 
	-> Result<(), error_handler::DataStructError<T>>
	{
		let previous_val = self.index_by_value.insert(next_value, self.next_index);
		if previous_val.is_some(){
			Err(error_handler::DataStructError::KeyAlreadyExists)
		}else{
			self.list.push(next_value);
			self.next_index += 1;
			Ok(())
		}
	}

	pub fn remove_by_value(&mut self, value_to_remove: T) 
	-> Result<(), error_handler::DataStructError<T>>
	{
		let optional_value_index = 
			self.index_by_value.remove(&value_to_remove);
		match optional_value_index{
			None => {
				Err(error_handler::DataStructError::ItemNotFoundInMap(value_to_remove))
			},
			Some(index_to_remove) => {
				self.next_index = index_to_remove;
				Ok(())
			}
		}
	}
}