
/// returns true if the value was found and removed
pub fn remove_by_value<T: PartialEq>(
	item_to_remove: &T, 
	list_to_remove_from: &mut Vec<T>
) -> bool
{
    let optional_index_to_remove = item_to_index(
		item_to_remove,
		list_to_remove_from
	);
    match optional_index_to_remove{
		Some(index_to_remove) => {
			list_to_remove_from.swap_remove(index_to_remove);
			true
		},
		None=> false
	}
}

pub fn item_to_index<T: PartialEq>(
	item_to_remove: &T, 
	list_to_remove_from: &Vec<T>
) -> Option<usize>
{
    list_to_remove_from.iter().position(|x| *x == *item_to_remove)
}