use crate::{prelude::*, output::error_handler};

use rand::Rng;

pub fn generate_board_by_vector_permutation(
    board: TileTypeBoard
) -> Result<TileTypeBoard, error_handler::BoardGenerationError>
{
    let solved_board_iterator = board.iter_filtered();
    let mut sorted_tiles=vec![];
    let mut sorted_grid_locations=vec![];
    for (sorted_grid_location, optional_tile) in solved_board_iterator{
        if let Some(tile) = optional_tile{
            sorted_grid_locations.push(sorted_grid_location);
            sorted_tiles.push(*tile);
        }else{
            return Err(error_handler::BoardGenerationError::GridError(GridError::IteratorYieldedNone));
        }
    }
    let permutation 
        = make_valid_permutation_out_of_vector(&sorted_tiles)?;
    let mut grid=Grid::new_with_default_values(
        *board.get_side_length(),
        IndexedValue::<TileType>::new(TileType::Wall)
    );
    let mut empty_grid_location = &GridLocation::default(); //there should always be an empty tile
    for (location, content) in sorted_grid_locations.iter().zip(permutation.iter()){
        grid.set(location, *content);
        if content.value == TileType::Empty {
            empty_grid_location = *location;
        }
    }
    let mut generated_board=
        TileTypeBoard::from_grid_and_empty_loc(&grid, empty_grid_location);
    generated_board.index_tile_of_type(TileType::Wall);
    Ok(generated_board)
}

fn make_valid_permutation_out_of_vector(sorted_vector: &Vec<IndexedValue<TileType>>) 
-> Result<Vec<IndexedValue<TileType>>, error_handler::BoardGenerationError> 
{
    let mut permutation_result
        = Err(error_handler::BoardGenerationError::VectorPermutationGenerationFailed);
    let mut rng = rand::thread_rng();
    let mut permutation ;
    let permutation_length = sorted_vector.len();
    
    for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
        //generate random permutation
        let mut cloned_sorted = sorted_vector.clone();
        let mut cloned_sorted_size = permutation_length;
        permutation = vec![];
        for _index in 0..sorted_vector.len(){
            let chosen_index=rng.gen_range(0..cloned_sorted_size);
            permutation.push(cloned_sorted.swap_remove(chosen_index));
            cloned_sorted_size -= 1;
        }
        if validate_and_attempt_solvability(sorted_vector, &mut permutation){
            permutation_result=Ok(permutation);
            break;
        }
    }

    permutation_result
}




/// NTS, IMPORTANT: this version only solves for no obstacle board with odd side length
/// tis but an example
fn validate_and_attempt_solvability(
    sorted_vector: &Vec<IndexedValue<TileType>>, 
    permutation: &mut Vec<IndexedValue<TileType>>
) 
-> bool 
{
    let mut wrong_placed = vec![];
    for ((item_index, sorted_value), permutation_value) 
        in sorted_vector.iter().enumerate().zip(permutation.iter()) 
    {
        if sorted_value != permutation_value{

            // can replace with a counter if I give up attempting solvability
            wrong_placed.push(
                IndexedValue::<IndexedValue::<TileType>>{
                    index: item_index,
                    value: *permutation_value
                }
            );
        }
    }
    if wrong_placed.len() % 2 == 0 {
        true
    }else{

        false
        // attempt_solvability(&mut wrong_placed, permutation)
    }
}

// /// tries to replace a wrong-placed with another wrong-placed that's shouldn't be in its place
// /// in order to make their amount even
// /// returns whether the attempt was successful
// fn attempt_solvability(wrong_placed: &mut Vec<IndexedValue::<usize>>, permutation: &mut Vec<TileType>)
// -> bool
// {
//     while let Some(wrong_placed_element) = wrong_placed.pop(){
//         let numeric_value = wrong_placed_element.value;
//         //in a solved board, numbers start at 1 at index 0
//         let correct_location=numeric_value-1;
//         //if it wasn't found, the corresponding number is correct 
//         //and thus switching would keep an odd number of wrong-placed
//         if let Ok(index_in_wrongs) = wrong_placed
//             .binary_search_by_key(
//                 &correct_location, 
//                 |&indexed_value| indexed_value.index
//             )
//         {
//             let elememt_in_correct_location 
//                 = wrong_placed.get(index_in_wrongs);
//             //we can unwrap directly since it was returned as Ok()
//             let number_in_correct_location = elememt_in_correct_location.unwrap().value;
//             //putting two wrong numbers in the right place keeps the number of wrongs odd
//             if number_in_correct_location != (wrong_placed_element.index - 1) {
//                 permutation.swap(
//                     wrong_placed_element.index, 
//                     elememt_in_correct_location.unwrap().index
//                 );
//                 break;
//             }
//         }
//     }
//     false
// }



    //TODO: TESTS FOR EMPTY WITH ODD LENGTH
    //attempt_solvability 
    /*
    examples:
    only one in wrong place (should be false)
     */
    //validate_and_attempt_solvability
    /*
    examples:
    sorted (should be true)
    two wrong places (should be true)
    only one in wrong place (should be false)
     */
    //make_valid_permutation_out_of_vector
    /*
    I should check that the returned one has an even number of tiles in wrong places
     */