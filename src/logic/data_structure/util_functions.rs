use std::hash::Hash;

use bevy::utils::HashMap;
use rand::Rng;

pub fn remove_all_items_with_keys<T: PartialEq + Eq + Hash + Copy, S>
    (map_to_remove_from: &mut HashMap<T, S>, other_vec: &Vec<T>)
{
    let mut keys: Vec<T> = map_to_remove_from.keys().copied().collect();
    for item in other_vec{
        if remove_by_value(item, &mut keys).is_some(){
            map_to_remove_from.remove(item);
        }
    }
}

pub fn get_single_key_value<T: PartialEq + Eq + Hash + Copy, S>
    (map_with_single_entry: &HashMap<T, S>) -> Option<(&T,&S)>
{
    let keys: Vec<T> = map_with_single_entry.keys().copied().collect();
    if let Some(key) = keys.into_iter().next(){
        map_with_single_entry.get_key_value(&key)
    }else{
        None
    }
}

pub fn remove_all_similar_items_from_former<T: PartialEq>(vec_to_remove_from: &mut Vec<T>, other_vec: &Vec<T>){
    for item in other_vec{
        remove_by_value(item, vec_to_remove_from);
    }
}

pub fn remove_by_value<T: PartialEq>(item_to_remove: &T, vec_to_remove_from: &mut Vec<T>) -> Option<T> {
    let optional_index_to_remove = item_to_index(item_to_remove, vec_to_remove_from);
    optional_index_to_remove.map(|index_to_remove| vec_to_remove_from.swap_remove(index_to_remove))
}

pub fn item_to_index<T: PartialEq>(item_to_find: &T, vec_to_find_in: &Vec<T>) -> Option<usize> {
    vec_to_find_in.iter().position(|x| *x == *item_to_find)
}

pub fn random_value<T: Copy>(vec_ref: &Vec<T>) -> T {
    let random_index = random_index(vec_ref);
    vec_ref[random_index]
}

pub fn random_index<T>(vec_ref: &Vec<T>) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..vec_ref.len())
}