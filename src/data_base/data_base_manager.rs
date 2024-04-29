use crate::prelude::*;
use crate::prelude::BoardGenerationError;
use crate::system::system_access::create_folder_if_none_exists_yet;


#[derive(Resource, Default)]
pub struct DataBaseManager{
	saved_layouts: Vec<Entity>,
}

pub struct DataBaseManagerPlugin;

impl Plugin for DataBaseManagerPlugin{
	fn build(&self, app: &mut App) {
		app.init_resource::<DataBaseManager>()
			.add_systems(Startup, read_system_text_files_into_db)
			.add_systems(Update, (
				save_to_data_base_and_system,
				remove_from_data_base_and_system,
				listen_to_db_clearing_request
			).in_set(InputSystemSets::InputHandling));
	}
}

fn read_system_text_files_into_db(
	mut db_manager: ResMut<DataBaseManager>,
	domain_board_query: Query<&DomainBoard>,
	mut commands: Commands
){
	if let Err(data_base_error) =
		read_system_text_files_into_db_inner(
			db_manager.as_mut(),
			&domain_board_query,
			&mut commands
		)
	{
		print_data_base_error(data_base_error);
	}
}

fn read_system_text_files_into_db_inner(
	db_manager: &mut DataBaseManager,
	domain_board_query: &Query<&DomainBoard>,
	commands: &mut Commands
) -> Result<(), DataBaseError>
{
	create_folder_if_none_exists_yet(FolderToAccess::SavedLayouts);
	let valid_text_file_names =
		get_all_valid_text_file_names_in_folder(FolderToAccess::SavedLayouts);

	for valid_text_file_name in valid_text_file_names{
		match domain_board_from_file(
			FolderToAccess::SavedLayouts,
			valid_text_file_name
		){
			Err(system_access_error) => {
				return Err(DataBaseError::SystemAccessError(system_access_error));
			},
			Ok(parsed_domain_board) => {
				match determine_board_quality(&parsed_domain_board){
					BoardQuality::Invalid => {
						return Err(DataBaseError::WallListDoesntMatchWallCount(
							parsed_domain_board.board_name.clone()
						));
					},
					_ => {
						//TODO: expand in the future
					}
				};
				
				wrap_to_data_base_error(db_manager.insert_layout_and_spawn_entity(
					&parsed_domain_board,
					domain_board_query,
					commands
				))?;
			}
		}
	}
	Ok(())
}

//TODO: expand in the future
fn determine_board_quality(parsed_domain_board: &DomainBoard) -> BoardQuality{
	if parsed_domain_board.wall_locations.len() != parsed_domain_board.board_props.wall_count as usize{
		BoardQuality::Invalid
	}else{
		BoardQuality::BestQuality
	}
}

fn save_to_data_base_and_system(
	mut event_writer: EventWriter<SuccessSavingToDB>,
	mut event_reader: EventReader<SaveToDB>,
	mut db_manager: ResMut<DataBaseManager>,
	domain_board_query: Query<&DomainBoard>,
	mut commands: Commands
){
	for save_request in event_reader.read(){
		match save_to_data_base_and_system_inner(
			save_request,
			&mut db_manager,
			&domain_board_query,
			&mut commands
		){
			Err(data_base_error) => {
				print_data_base_error(data_base_error);
			},
			Ok(index_saved_to) => {
				event_writer.send(SuccessSavingToDB(index_saved_to));
			}
		}
	}
}

fn save_to_data_base_and_system_inner(
	save_request: &SaveToDB,
	db_manager: &mut DataBaseManager,
	domain_board_query: &Query<&DomainBoard>,
	commands: &mut Commands
) -> Result<SavedLayoutIndex, DataBaseError>
{
	let layout_content_string = ron::ser::to_string_pretty(
		&save_request.0, ron::ser::PrettyConfig::default()).unwrap();
	let layout_name_string = db_manager.generate_default_name_for_board();
	write_to_file(
		FolderToAccess::SavedLayouts,
		layout_name_string.0,
		layout_content_string
	).unwrap();

	wrap_to_data_base_error(db_manager.insert_layout_and_spawn_entity(
		&save_request.0,
		domain_board_query,
		commands
	))
}

fn remove_from_data_base_and_system(
	mut event_writer: EventWriter<SuccessRemovingFromDB>,
	mut event_reader: EventReader<RemoveFromDB>,
	mut db_manager: ResMut<DataBaseManager>,
	domain_board_query: Query<&DomainBoard>,
	mut commands: Commands
){
	for removal_request in event_reader.read(){
		match remove_from_data_base_and_system_inner(
			&removal_request.0,
			db_manager.as_mut(),
			&domain_board_query,
			&mut commands
		){
			Err(data_base_error) => {
				print_data_base_error(data_base_error);
			},
			Ok(_) => {
				event_writer.send(SuccessRemovingFromDB(removal_request.0));
			}
		}
	}
}

fn remove_from_data_base_and_system_inner(
	layout_index: &SavedLayoutIndex,
	db_manager: &mut DataBaseManager,
	domain_board_query: &Query<&DomainBoard>,
	commands: &mut Commands
) -> Result<(), DataBaseError>
{
	match db_manager.remove_layout_by_index_and_despawn_entity(layout_index, domain_board_query, commands){
		Some(removed_domain_board_name)=> {
			if delete_text_file(
				FolderToAccess::SavedLayouts,
				removed_domain_board_name.0.clone()
			).is_err()
			{
				Err(DataBaseError::SystemAccessError
						(SystemAccessError::CouldntFindFile(FileName(removed_domain_board_name.0.clone()))))
			}else{
				Ok(())
			}
		},
		None => Ok(())
	}
}

fn listen_to_db_clearing_request(
	mut event_writer: EventWriter<SuccessClearingDB>,
	mut event_reader: EventReader<ClearDB>,
	mut db_manager: ResMut<DataBaseManager>
){
	for _clear_request in event_reader.read() {
		match clear_db(&mut db_manager){
			Err(data_base_error) => {
				print_data_base_error(data_base_error);
			},
			Ok(_) => {
				event_writer.send(SuccessClearingDB);
			}
		}
	}
}

fn clear_db(
	db_manager: &mut DataBaseManager
) -> Result<(), DataBaseError>
{
	db_manager.saved_layouts = vec!();
	create_folder_if_none_exists_yet(FolderToAccess::SavedLayouts);
	let valid_text_file_names =
		get_all_valid_text_file_names_in_folder(FolderToAccess::SavedLayouts);

	for valid_text_file_name in valid_text_file_names{
		let valid_text_file_name_excluding_postfix =
			&valid_text_file_name[..valid_text_file_name.len()-4];
		let file_deletion_result = delete_text_file(
			FolderToAccess::SavedLayouts,
			String::from(valid_text_file_name_excluding_postfix)
		);
		if file_deletion_result.is_err(){
			return Err(DataBaseError::SystemAccessError
				(SystemAccessError::CouldntFindFile(FileName(valid_text_file_name))));
		}
	}
	Ok(())
}

fn wrap_to_data_base_error<T>(result: Result<T, GridError>) -> Result<T, DataBaseError> {
	match result {
		Err(grid_error) => {
			Err(DataBaseError::CouldntBuildTileBoardFromWallLocations
					(BoardGenerationError::GridError(grid_error)))
		},
		Ok(value) => Ok(value)
	}
}

impl DataBaseManager{
	pub fn insert_layout_and_spawn_entity(
		&mut self,
		domain_board: &DomainBoard,
		domain_board_query: &Query<&DomainBoard>,
		commands: &mut Commands
	) -> Result<SavedLayoutIndex, GridError>
	{
		let newborn_entity = DataBaseManager::spawn_layout_entity(domain_board, commands)?;
		Ok(self.insert_layout(domain_board, domain_board_query, newborn_entity))
	}

	fn spawn_layout_entity(domain_board: &DomainBoard, commands: &mut Commands) -> Result<Entity, GridError>{
		Ok(commands.spawn(SavedLayoutBundle{
			domain_board: domain_board.clone(),
			tile_board: TileBoard::try_from_domain_board(domain_board)?
		}).id())
	}

	fn insert_layout(
		&mut self,
		domain_board: &DomainBoard,
		domain_board_query: &Query<&DomainBoard>,
		entity: Entity
	) -> SavedLayoutIndex
	{
		let index = self.saved_layouts.partition_point(|saved_layout| {
			domain_board_query.get(*saved_layout).unwrap().board_name < domain_board.board_name
		});
		self.saved_layouts.insert(index, entity);
		SavedLayoutIndex(index)
	}

	pub fn remove_layout_by_index_and_despawn_entity(
		&mut self,
		index: &SavedLayoutIndex,
		domain_board_query: &Query<&DomainBoard>,
		commands: &mut Commands
	)
		-> Option<DomainBoardName>
	{
		let index_value = index.0;
		if index_value < self.saved_layouts.len(){
			let removed_entity = self.saved_layouts.remove(index.0);
			let removed_layout_name = domain_board_query.get(removed_entity).unwrap().board_name.clone();
			commands.entity(removed_entity).despawn_recursive();
			Some(removed_layout_name)
		}else{
			None
		}
	}
	
	pub fn try_get_layout_ref(&self, index: &SavedLayoutIndex) -> Option<&Entity> {
		self.saved_layouts.get(index.0)
	}

	pub fn get_saved_layouts_ref(&self) -> &Vec<Entity>{
		&self.saved_layouts
	}
	
	pub fn generate_default_name_for_board(&self) -> DomainBoardName {
		DomainBoardName(format!("layout_{:?}", self.saved_layouts.len()))
	}
}