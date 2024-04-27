use crate::prelude::*;
use crate::system::system_access::create_folder_if_none_exists_yet;


#[derive(Resource, Default)]
pub struct DataBaseManager{
	saved_layouts: Vec<DomainBoard>,
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
	mut db_manager: ResMut<DataBaseManager>
){
	if let Err(board_loading_error) = read_system_text_files_into_db_inner(db_manager.as_mut()){
		print_board_loading_error(board_loading_error);
	}
}

fn read_system_text_files_into_db_inner(
	db_manager: &mut DataBaseManager
) -> Result<(), BoardLoadingError>
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
				return Err(BoardLoadingError::SystemAccessError(system_access_error));
			},
			Ok(parsed_domain_board) => {
				match determine_board_quality(&parsed_domain_board){
					BoardQuality::Invalid => {
						return Err(BoardLoadingError::WallListDoesntMatchWallCount(
							DomainBoardName(parsed_domain_board.board_name.clone())
						));
					},
					_ => {
						//TODO: expand in the future
					}
				};
				db_manager.insert_layout(&parsed_domain_board);
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
	mut db_manager: ResMut<DataBaseManager>
){
	for save_request in event_reader.read(){
		let layout_content_string = ron::ser::to_string_pretty(
			&save_request.0, ron::ser::PrettyConfig::default()).unwrap();
		let layout_name_string = db_manager.generate_default_name_for_board();
		write_to_file(
			FolderToAccess::SavedLayouts,
			layout_name_string.clone(),
			layout_content_string
		).unwrap();

		let index_saved_to = db_manager.as_mut().insert_layout(&save_request.0);
		event_writer.send(SuccessSavingToDB(index_saved_to));
	}
}

fn remove_from_data_base_and_system(
	mut event_writer: EventWriter<SuccessRemovingFromDB>,
	mut event_reader: EventReader<RemoveFromDB>,
	mut db_manager: ResMut<DataBaseManager>
){
	for removal_request in event_reader.read(){
		match remove_from_data_base_and_system_inner(&removal_request.0, db_manager.as_mut()){
			Err(system_access_error) => {
				print_system_access_error(system_access_error);
			},
			Ok(_) => {
				event_writer.send(SuccessRemovingFromDB(removal_request.0));
			}
		}
	}
}

fn remove_from_data_base_and_system_inner(
	layout_index: &SavedLayoutIndex,
	db_manager: &mut DataBaseManager
) -> Result<(), SystemAccessError>
{
	match db_manager.remove_layout_by_index(layout_index){
		Some(removed_domain_board)=> {
			if delete_text_file(
				FolderToAccess::SavedLayouts,
				removed_domain_board.board_name.clone()
			).is_err()
			{
				Err(SystemAccessError::CouldntFindFile(FileName(removed_domain_board.board_name.clone())))
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
			Err(system_access_error) => {
				print_system_access_error(system_access_error);
			},
			Ok(_) => {
				event_writer.send(SuccessClearingDB);
			}
		}
	}
}

fn clear_db(
	db_manager: &mut DataBaseManager
) -> Result<(), SystemAccessError>
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
			return Err(SystemAccessError::CouldntFindFile(FileName(valid_text_file_name)));
		}
	}
	Ok(())
}

impl DataBaseManager{
	pub fn insert_layout(&mut self, layout: &DomainBoard) -> SavedLayoutIndex{
		let index = self.saved_layouts.partition_point(|saved_layout| 
			saved_layout.board_name < layout.board_name
		);
		self.saved_layouts.insert(index, layout.clone());
		SavedLayoutIndex(index)
	}

	pub fn remove_layout_by_index(&mut self, index: &SavedLayoutIndex) -> Option<DomainBoard>{
		let index_value = index.0;
		if index_value < self.saved_layouts.len(){
			Some(self.saved_layouts.remove(index.0))	
		}else{
			None
		}
	}
	
	pub fn try_get_layout_ref(&self, index: &SavedLayoutIndex) -> Option<&DomainBoard> {
		self.saved_layouts.get(index.0)
	}

	pub fn get_saved_layouts_ref(&self) -> &Vec<DomainBoard>{
		&self.saved_layouts
	}
	
	pub fn generate_default_name_for_board(&self) -> String {
		format!("layout_{:?}", self.saved_layouts.len())
	}
}