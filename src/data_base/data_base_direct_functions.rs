use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use crate::prelude::*;

impl DataBaseManager {
    pub fn insert_layout_and_spawn_entity(
        &mut self,
        new_domain_board_name: &DomainBoardName,
        new_domain_board: &DomainBoard,
        domain_board_query: &Query<(Entity, &DomainBoardName, &DomainBoard)>,
        commands: &mut Commands
    ) -> Result<SavedLayoutIndexInDifficultyVec, GridError>
    {
        let newborn_entity =
            DataBaseManager::spawn_layout_entity(new_domain_board_name, new_domain_board, commands)?;
        Ok(self.insert_layout(
            newborn_entity,
            new_domain_board_name,
            &new_domain_board.board_props.board_difficulty,
            domain_board_query,
        ))
    }

    pub fn spawn_layout_entity(
        domain_board_name: &DomainBoardName,
        domain_board: &DomainBoard,
        commands: &mut Commands
    ) -> Result<Entity, GridError>
    {
        Ok(commands.spawn(SavedLayoutBundle {
            domain_board_name: domain_board_name.clone(),
            domain_board: domain_board.clone(),
            tile_board: TileBoard::try_from_domain_board(domain_board)?
        }).id())
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
            DataBaseManager::get_partition_board_in_vec_by_name(
                layouts_of_difficulty,
                new_domain_board_name,
                domain_board_query
            )
        }else{
            self.saved_layouts.insert(*new_domain_board_difficulty, vec!());
            0
        };

        let mut layouts_of_same_difficult_mut =
            self.saved_layouts.get_mut(new_domain_board_difficulty).unwrap();
        layouts_of_same_difficult_mut.insert(index_in_dif_vec, new_board_entity);

        SavedLayoutIndexInDifficultyVec {
            difficulty: *new_domain_board_difficulty,
            index_in_own_dif: index_in_dif_vec
        }
    }

    fn get_partition_board_in_vec_by_name(
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
        for (_dif, vec) in self.saved_layouts.iter(){
            combined_length += vec.len();
        }
        combined_length
    }
    
    pub fn get_layouts_count_by_difficulty(&self, board_difficulty: &BoardDifficulty) -> Option<usize> {
        Some(self.saved_layouts.get(board_difficulty)?.len())
    }

    pub fn generate_default_name_for_board(&self) -> DomainBoardName {
        DomainBoardName(format!("layout_{:?}", self.saved_layouts.len()))
    }
}