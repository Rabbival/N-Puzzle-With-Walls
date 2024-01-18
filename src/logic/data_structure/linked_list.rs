use sorted_vec::SortedVec;

pub struct LinkedList<T: Ord>{
	list: SortedVec<T>,
	next_index: usize
}

impl<T: Ord> LinkedList<T>{
	pub fn new() -> LinkedList<T>{
		Self { 
			list: SortedVec::new(),
			next_index: 0
		}
	}
	
	pub fn push(&mut self, next_value: T){
		self.list.insert(next_value);
		self.next_index += 1;
	}

	pub fn remove_by_value(&mut self, value_to_remove: T){

		// TODO: write a binary search that gets start and end indexes

		let search_result = self.list.binary_search(&value_to_remove);
		match search_result{
			Err(_) => {
				// TODO: make an error type for that
			},
			Ok(found_index) =>{
				self.next_index = found_index;
			}
		}
	}
}