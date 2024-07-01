use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use crate::prelude::*;

impl DataBaseManager {
    pub fn insert_layout_and_spawn_entity(
        &mut self,
        new_domain_board_name: &DomainBoardName,
        new_domain_board: &DomainBoard,
        domain_board_query: &Query<(Entity, &DomainBoardName, &DomainBoard)>,
        commands: &mut Commands
    ) -> SavedLayoutIndexInDifficultyVec
    {
        let newborn_entity =
            Self::spawn_layout_entity(new_domain_board_name, new_domain_board, commands);
        self.insert_layout(
            newborn_entity,
            new_domain_board_name,
            &new_domain_board.board_props.board_difficulty,
            domain_board_query,
        )
    }

    pub fn spawn_layout_entity(
        domain_board_name: &DomainBoardName,
        domain_board: &DomainBoard,
        commands: &mut Commands
    ) -> Entity
    {
        commands.spawn(SavedLayoutBundle {
            domain_board_name: domain_board_name.clone(),
            domain_board: domain_board.clone(),
            tile_board: TileBoard::from_grid(&domain_board.grid)
        }).id()
    }

    pub fn insert_layout(
        &mut self,
        new_board_entity: Entity,
        new_domain_board_name: &DomainBoardName,
        new_domain_board_difficulty: &BoardDifficulty,
        domain_board_query: &Query<(Entity, &DomainBoardName, &DomainBoard)>,
    ) -> SavedLayoutIndexInDifficultyVec
    {
        let layouts_of_same_difficulty = self.saved_layouts.get(new_domain_board_difficulty);
        let index_in_dif_vec = if let Some(layouts_of_difficulty) = layouts_of_same_difficulty{
            DataBaseManager::get_partition_point_in_vec_by_name(
                layouts_of_difficulty,
                new_domain_board_name,
                domain_board_query
            )
        }else{
            self.saved_layouts.insert(*new_domain_board_difficulty, vec!());
            0
        };

        let layouts_of_same_difficulty_mut =
            self.saved_layouts.get_mut(new_domain_board_difficulty).unwrap();
        layouts_of_same_difficulty_mut.insert(index_in_dif_vec, new_board_entity);

        SavedLayoutIndexInDifficultyVec {
            difficulty: *new_domain_board_difficulty,
            index_in_own_dif: index_in_dif_vec
        }
    }

    fn get_partition_point_in_vec_by_name(
        layouts_of_difficulty: &Vec<Entity>,
        new_domain_board_name: &DomainBoardName,
        domain_board_query: &Query<(Entity, &DomainBoardName, &DomainBoard)>,
    ) -> usize
    {
        layouts_of_difficulty.partition_point(|saved_layout| {
            if let Ok((_, existing_domain_board_name, _)) =
                domain_board_query.get(*saved_layout)
            {
                existing_domain_board_name < new_domain_board_name
            }else{
                false
            }
        })
    }

    pub fn remove_layout_by_index_and_despawn_entity(
        &mut self,
        index: &SavedLayoutIndexInDifficultyVec,
        domain_board_name_query: &Query<&DomainBoardName>,
        commands: &mut Commands
    )
        -> Option<DomainBoardName>
    {
        let difficulty_vec = self.saved_layouts.get_mut(&index.difficulty)?;
        let index_in_difficulty = index.index_in_own_dif;
        if index_in_difficulty < difficulty_vec.len() {
            let removed_entity = difficulty_vec.remove(index_in_difficulty);
            if let Ok(removed_layout_name) = domain_board_name_query.get(removed_entity){
                commands.entity(removed_entity).despawn_recursive();
                return Some(removed_layout_name.clone());
            }
        }
        None
    }

    pub fn try_get_layout_ref(&self, index: &SavedLayoutIndexInDifficultyVec) -> Option<&Entity> {
        self.saved_layouts.get(&index.difficulty)?.get(index.index_in_own_dif)
    }

    pub fn get_saved_layouts_of_all_difficulties_count(&self) -> usize {
        let mut combined_length = 0;
        for (_dif, vec) in &self.saved_layouts{
            combined_length += vec.len();
        }
        combined_length
    }

    pub fn get_layouts_count_by_difficulty(&self, board_difficulty: &BoardDifficulty) -> Option<usize> {
        Some(self.saved_layouts.get(board_difficulty)?.len())
    }

    pub fn generate_unique_default_name_for_board(&self, domain_board_names_query: &Query<&DomainBoardName>) -> DomainBoardName {
        let mut new_layout_number = self.get_saved_layouts_of_all_difficulties_count();
        let mut new_board_name = DomainBoardName(format!("layout-{:?}", new_layout_number));
        while DataBaseManager::domain_board_name_already_exists(&new_board_name, domain_board_names_query){
            new_layout_number += 1;
            new_board_name = DomainBoardName(format!("layout-{:?}", new_layout_number));
        }
        new_board_name
    }
}

//already exists check functions
impl DataBaseManager{
    pub fn domain_board_already_exists(
        domain_boards_query: &Query<(&DomainBoard, &DomainBoardName)>,
        game_board_grid: &Grid<Tile>
    ) -> Option<ExistingWallLayoutName> {
        for (domain_board, domain_board_name) in domain_boards_query{
            if domain_board.grid == *game_board_grid {
                return Some(ExistingWallLayoutName(domain_board_name.0.clone()));
            }
        }
        None
    }

    //TODO: look for layout in all difficulties and return Some(SavedLayoutIndexInDifficultyVec) if found
    // then save that information in NewbornDomainBoardName.already_exists and use that
    // to call remove_layout_by_index_and_despawn_entity if it's a Some
    pub fn domain_board_name_already_exists(
        domain_board_name_to_check: &DomainBoardName,
        domain_boards_query: &Query<&DomainBoardName>
    ) -> bool {
        for domain_board_name in domain_boards_query{
            if *domain_board_name_to_check == *domain_board_name {
                return true;
            }
        }
        false
    }
}