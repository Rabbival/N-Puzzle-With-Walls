use rand::Rng;

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
	item_to_find: &T, 
	list_to_find_in: &Vec<T>
) -> Option<usize>
{
    list_to_find_in.iter().position(|x| *x == *item_to_find)
}

pub fn random_value<T:Copy>(list_ref: &Vec<T>) -> T {
	let mut rng = rand::thread_rng();
	let random_index = rng.gen_range(0..list_ref.len());
	list_ref[random_index]
}