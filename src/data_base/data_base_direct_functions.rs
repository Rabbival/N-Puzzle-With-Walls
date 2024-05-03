use crate::prelude::*;

impl DataBaseManager {
    pub fn insert_layout_and_spawn_entity(
        &mut self,
        domain_board_name: &DomainBoardName,
        domain_board: &DomainBoard,
        domain_board_query: &Query<(Entity, &DomainBoardName), With<DomainBoard>>,
        commands: &mut Commands
    ) -> Result<SavedLayoutIndex, GridError>
    {
        let newborn_entity =
            DataBaseManager::spawn_layout_entity(domain_board_name, domain_board, commands)?;
        Ok(self.insert_layout(domain_board_name, domain_board_query, newborn_entity))
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
        domain_board_name: &DomainBoardName,
        domain_board_query: &Query<(Entity, &DomainBoardName), With<DomainBoard>>,
        entity: Entity
    ) -> SavedLayoutIndex
    {
        let index = self.saved_layouts.partition_point(|saved_layout| {
            if let Ok((_, existing_domain_board_name)) = domain_board_query.get(*saved_layout){
                existing_domain_board_name < domain_board_name
            }else{
                false
            }
        });
        self.saved_layouts.insert(index, entity);
        SavedLayoutIndex(index)
    }

    pub fn remove_layout_by_index_and_despawn_entity(
        &mut self,
        index: &SavedLayoutIndex,
        domain_board_name_query: &Query<&DomainBoardName>,
        commands: &mut Commands
    )
        -> Option<DomainBoardName>
    {
        let index_value = index.0;
        if index_value < self.saved_layouts.len() {
            let removed_entity = self.saved_layouts.remove(index.0);
            if let Ok(removed_layout_name) = domain_board_name_query.get(removed_entity){
                commands.entity(removed_entity).despawn_recursive();
                return Some(removed_layout_name.clone());
            }
        }
        None
    }

    pub fn try_get_layout_ref(&self, index: &SavedLayoutIndex) -> Option<&Entity> {
        self.saved_layouts.get(index.0)
    }

    pub fn get_saved_layouts_ref(&self) -> &Vec<Entity> {
        &self.saved_layouts
    }

    pub fn generate_default_name_for_board(&self) -> DomainBoardName {
        DomainBoardName(format!("layout_{:?}", self.saved_layouts.len()))
    }
}