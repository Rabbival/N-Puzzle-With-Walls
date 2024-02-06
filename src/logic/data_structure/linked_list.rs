use std::fmt::{Debug, Formatter};
use crate::output::error_handler;

/// a data structure for which if a node is removed all next ones follow
pub struct LinkedList<T: Ord + Copy>{
	list: Vec<T>,
	next_index: usize
}

impl<T: Ord + Copy> LinkedList<T>{
	pub fn new() -> LinkedList<T>{
		Self { 
			list: vec![],
			next_index: 0
		}
	}

	pub fn is_empty(&self) -> bool{
		self.next_index == 0
	}
	
	pub fn push(&mut self, next_value: T)
	{
		self.list.insert(self.next_index, next_value);
		self.next_index += 1;
	}

	pub fn remove_by_value(&mut self, ref_to_value_to_remove: &T)
	-> Result<(), error_handler::DataStructError<T>>
	{
		let optional_val_to_remove_index =
			self.item_to_index(ref_to_value_to_remove);
		match optional_val_to_remove_index{
			None => {
				Err(error_handler::DataStructError::ItemNotFound(*ref_to_value_to_remove))
			},
			Some(index_to_remove) => {
				self.next_index = index_to_remove;
				Ok(())
			}
		}
	}

	fn item_to_index(&self, item_to_find: &T) -> Option<usize> {
		for (item_index, item) in self.list.iter().enumerate(){
			if item_index >= self.next_index{
				return None;
			}else{
				if item == item_to_find {
					return Some(item_index);
				}
			}
		}
		None
	}
}

impl<T: Ord + Copy + Debug> Debug for LinkedList<T>{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Linked List")
			.field("list", &self.list)
			.finish()
	}
}