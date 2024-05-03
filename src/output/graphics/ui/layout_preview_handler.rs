use crate::prelude::*;

pub struct LayoutPreviewHandlerPlugin;

impl Plugin for LayoutPreviewHandlerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            set_node_to_new_layout_preview_listener.after(set_slot_layout_preview)
        );
    }
}

fn set_node_to_new_layout_preview_listener(
   mut event_reader: EventReader<SetNodeToPreviewLayout>
){
    for _event_temp_name in event_reader.read(){
        
    }
}