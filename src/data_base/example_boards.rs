use crate::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct ExampleBoards(pub HashMap<DomainBoardName, BoardProperties>);

pub struct ExampleBoardsPlugin;

impl Plugin for ExampleBoardsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExampleBoards>().add_systems(
            Startup,
            (initialize_example_boards, spawn_example_boards).chain(),
        );
    }
}

fn initialize_example_boards(mut example_boards: ResMut<ExampleBoards>) {
    example_boards.0.insert(
        DomainBoardName(String::from("Tiny Introduction")),
        BoardProperties {
            board_difficulty: BoardDifficulty::Easy,
            size: BoardSize::Tiny,
            wall_count: 0,
            empty_count: 1,
            generation_method: BoardGenerationMethod::Auto,
            tree_traveller_type: Default::default(),
        },
    );
    example_boards.0.insert(
        DomainBoardName(String::from("Classic 15")),
        BoardProperties {
            board_difficulty: BoardDifficulty::Medium,
            size: BoardSize::Small,
            wall_count: 0,
            empty_count: 1,
            generation_method: BoardGenerationMethod::Auto,
            tree_traveller_type: Default::default(),
        },
    );
    example_boards.0.insert(
        DomainBoardName(String::from("3 Blocks")),
        BoardProperties {
            board_difficulty: BoardDifficulty::Medium,
            size: BoardSize::Small,
            wall_count: 3,
            empty_count: 1,
            generation_method: BoardGenerationMethod::Auto,
            tree_traveller_type: GridTravellerType::BFS,
        },
    );
    example_boards.0.insert(
        DomainBoardName(String::from("For The Experienced")),
        BoardProperties {
            board_difficulty: BoardDifficulty::Hard,
            size: BoardSize::Large,
            wall_count: 8,
            empty_count: 1,
            generation_method: BoardGenerationMethod::Auto,
            tree_traveller_type: GridTravellerType::DFS,
        },
    );
    example_boards.0.insert(
        DomainBoardName(String::from("A True Challenge")),
        BoardProperties {
            board_difficulty: BoardDifficulty::Hard,
            size: BoardSize::Giant,
            wall_count: 20,
            empty_count: 1,
            generation_method: BoardGenerationMethod::Auto,
            tree_traveller_type: GridTravellerType::BFS,
        },
    );
}

fn spawn_example_boards(
    mut save_to_board_event_writer: EventWriter<SaveToDB>,
    example_boards: Res<ExampleBoards>,
    db_manager: Res<DataBaseManager>,
    board_name_query: Query<&DomainBoardName>,
) {
    if let Err(board_gen_error) = spawn_example_boards_inner(
        &mut save_to_board_event_writer,
        &example_boards,
        &db_manager,
        &board_name_query,
    ) {
        print_board_generation_error(board_gen_error);
    }
}

fn spawn_example_boards_inner(
    save_to_board_event_writer: &mut EventWriter<SaveToDB>,
    example_boards: &ExampleBoards,
    db_manager: &DataBaseManager,
    board_name_query: &Query<&DomainBoardName>,
) -> Result<(), BoardGenerationError> {
    for (board_name, board_props) in &example_boards.0 {
        let grid_side_length = board_props.size.to_grid_side_length();
        let mut tile_board = TileBoard::new(grid_side_length);
        generate_solved_board_inner(board_props, &mut tile_board)?;
        let shuffled_board =
            brute_force_generate_game_board(&tile_board, board_props.size.to_random_turns_range())?;
        save_to_board_event_writer.send(SaveToDB {
            name: board_name.clone(),
            board: DomainBoard {
                board_props: *board_props,
                grid: shuffled_board.grid,
            },
            existing_boards_with_same_name_and_difficulty: db_manager
                .get_existing_boards_with_same_name_and_difficulty(board_name, board_name_query),
        });
    }
    Ok(())
}
