use crate::{prelude::*, output::{print_to_console, error_handler}, costume_event::move_tile_event};

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2
}

pub struct MouseInputHandlerPlugin;

impl Plugin for MouseInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorPosition>()
            .add_systems(Update, (update_cursor, listen_for_mouse_click).chain());
    }
}

fn update_cursor(
    mut cursor: ResMut<CursorPosition>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, transform) = camera.single();

    if let Some(screen_position) = window.cursor_position() {
        let world_position = camera
            .viewport_to_world(transform, screen_position)
            .unwrap()
            .origin
            .truncate();
        cursor.world_position = world_position;
    }
}

fn listen_for_mouse_click(
    mut logic_event_writer: EventWriter<move_tile_event::SwitchTilesLogic>,
    mouse: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    game_board_query: Query<&TileTypeBoard, (With<GameBoard>, Without<SolvedBoard>)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Err(input_error) = 
        handle_mouse_click(
                &mut logic_event_writer,
                cursor_position.world_position, 
                game_board_query.single()
            )
        {
            print_to_console::print_tile_move_error(input_error);
        }
    }
}

fn handle_mouse_click(
    logic_event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>,
    cursor_position: Vec2,
    game_board: &TileTypeBoard,
) -> Result<(), error_handler::TileMoveError>
{
    if game_board.ignore_player_input{
        return Err(error_handler::TileMoveError::BoardFrozenToPlayer(String::from("board locked")));
    }
    if let Some(optional_occupied_tile_location) = GridLocation::from_world(&game_board.grid, cursor_position) {
        if !game_board.occupied(&optional_occupied_tile_location)? {
            return Err(error_handler::TileMoveError::PressedEmptySlot(String::from("pressed an empty slot")));
        }
        let occupied_tile_location=optional_occupied_tile_location;
        let optional_empty_neighbor_location= 
            game_board.get_empty_neighbor(&occupied_tile_location)?;
        if optional_empty_neighbor_location.is_none(){
            return Err(error_handler::TileMoveError::NoEmptyNeighbor(String::from("no empty neighbor")));
        }
        let empty_neighbor_location=optional_empty_neighbor_location.unwrap();

        logic_event_writer.send(move_tile_event::SwitchTilesLogic{
            occupied_tile_location,
            empty_tile_location: empty_neighbor_location
        });

        Ok(())
    }else{
        Err(error_handler::TileMoveError::IndexOutOfGridBounds(String::from("index out of grid bounds!")))
    }
}


#[cfg(test)]
mod tests {
    use crate::logic::board_building::solved_board_builder;

    use super::*;

    #[test]
    fn test_input_validation(){
        let mut app = App::new();
        app
            .add_event::<move_tile_event::SwitchTilesLogic>()
            .add_systems(Update, test_input_validation_inner)
        ;
        app.update();
    }

    fn test_input_validation_inner(mut event_writer: EventWriter<move_tile_event::SwitchTilesLogic>) {
        assert!(test_index_out_of_bound(
            Vec2::new(-100.0, -100.0),
            &mut event_writer
        ));
        assert!(test_index_out_of_bound(
            Vec2::new(
                DEFAULT_BOARD_SIDE_LENGTH as f32 * ATLAS_CELL_SQUARE_SIZE, 
                DEFAULT_BOARD_SIDE_LENGTH as f32 * ATLAS_CELL_SQUARE_SIZE
            ),
            &mut event_writer
        ));
    }

    fn test_index_out_of_bound(
        position_to_check: Vec2,
        event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>
    )-> bool
    {
        let mut board=TileTypeBoard::default();
        board.ignore_player_input=false;
        let location_search_outcome=
            handle_mouse_click(
                event_writer,
                position_to_check, 
                &board,
            );
        match location_search_outcome{
            Err(error_handler::TileMoveError::IndexOutOfGridBounds(_))=> true,
            _ => false
        }
    }
    

    #[test]
    fn test_board_freezing(){
        let mut app = App::new();
        app
            .add_event::<move_tile_event::SwitchTilesLogic>()
            .add_systems(Update, test_board_freezing_inner)
        ;
        app.update();
    }

    fn test_board_freezing_inner(mut event_writer: EventWriter<move_tile_event::SwitchTilesLogic>) {
        assert!(test_frozen_board(&mut event_writer));
    }

    fn test_frozen_board(event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>)-> bool{
        let location_validation_outcome=
            handle_mouse_click(
                event_writer,
                Vec2::default(), 
                &TileTypeBoard::default(), //locked by default
            );
        match location_validation_outcome{
            Err(TileMoveError::BoardFrozenToPlayer(_))=> true,
            _ => false
        }
    }


    #[test]
    fn test_valid_location(){
        let mut app = App::new();
        app
            .add_event::<move_tile_event::SwitchTilesLogic>()
            .add_systems(Update, test_valid_location_inner)
        ;
        app.update();
    }

    fn test_valid_location_inner(mut event_writer: EventWriter<move_tile_event::SwitchTilesLogic>){
        assert!(test_no_tile_in_cell(&mut event_writer));
        assert!(test_empty_slot(&mut event_writer));
        assert!(test_no_empty_neighbor(&mut event_writer));
    }

    fn test_no_tile_in_cell(event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>)-> bool{
        let mut board=TileTypeBoard::default();
        board.ignore_player_input=false;
        let location_validation_outcome=
            handle_mouse_click(
                event_writer,
                Vec2::default(), 
                &board,
            );

        println!("{:?}", location_validation_outcome);

        match location_validation_outcome{
            Err(TileMoveError::NoTileInCell(_))=> true,
            _ => false
        }
    }

    fn test_empty_slot(event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>)-> bool{
        let mut board=TileTypeBoard::default();
        board.ignore_player_input=false;
        board.set(&GridLocation::new(0, 0), TileType::new(None));
        let location_validation_outcome=
            handle_mouse_click(
                event_writer,
                Vec2::default(), 
                &board,
            );

        println!("{:?}", location_validation_outcome);

        match location_validation_outcome{
            Err(TileMoveError::PressedEmptySlot(_))=> true,
            _ => false
        }
    }

    fn test_no_empty_neighbor(event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>)-> bool{
        let mut board: TileTypeBoard=solved_board_builder::generate_solved_board(DEFAULT_BOARD_SIDE_LENGTH);
        board.ignore_player_input=false;
        let empty_tile_location=board.empty_tile_location;
        board.set(&empty_tile_location, TileType::new(Some(16)));
        let location_validation_outcome=
            handle_mouse_click(
                event_writer,
                Vec2::default(), 
                &board,
            );
        match location_validation_outcome{
            Err(TileMoveError::NoEmptyNeighbor(_))=> true,
            _ => false
        }
    }
}