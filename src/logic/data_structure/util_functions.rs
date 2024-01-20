use rand::Rng;

pub fn remove_by_value<T: PartialEq>(item_to_remove: &T, list_to_remove_from: &mut Vec<T>) -> Option<T> {
    let optional_index_to_remove = item_to_index(item_to_remove, list_to_remove_from);
    match optional_index_to_remove {
        Some(index_to_remove) => {
            Some(list_to_remove_from.swap_remove(index_to_remove))
        }
        None => None,
    }
}

pub fn item_to_index<T: PartialEq>(item_to_find: &T, list_to_find_in: &Vec<T>) -> Option<usize> {
    list_to_find_in.iter().position(|x| *x == *item_to_find)
}

pub fn random_value<T: Copy>(list_ref: &Vec<T>) -> T {
    let random_index = random_index(list_ref);
    list_ref[random_index]
}

pub fn random_index<T>(list_ref: &Vec<T>) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..list_ref.len())
}
