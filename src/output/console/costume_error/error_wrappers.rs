use crate::prelude::{BoardGenerationError, GridError, TileMoveError};

pub fn wrap_grid_error_in_tile_move_error<T>(result: Result<T, GridError>)
                                             -> Result<T, TileMoveError>{
    match result {
        Err(grid_error) => {
            Err(TileMoveError::GridError(grid_error))
        },
        Ok(value) => Ok(value)
    }
}

pub fn wrap_grid_error_in_tile_board_gen_error<T: Copy>(result: &Result<T, GridError>)
                                                    -> Result<T, BoardGenerationError>{
    match result {
        Err(grid_error) => {
            Err(BoardGenerationError::GridError(*grid_error))
        },
        Ok(value) => Ok(*value)
    }
}

pub fn wrap_grid_error_in_tile_board_gen_error_owned<T>(result: Result<T, GridError>)
                                                    -> Result<T, BoardGenerationError>{
    match result {
        Err(grid_error) => {
            Err(BoardGenerationError::GridError(grid_error))
        },
        Ok(value) => Ok(value)
    }
}