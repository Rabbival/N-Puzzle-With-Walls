use crate::prelude::*;


pub struct AreYouSureMessagePlugin;

impl Plugin for AreYouSureMessagePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                spawn_are_you_sure_message
            )
            .add_systems(
                Update,
                listen_for_are_you_sure_message_requests
            );
    }
}

fn spawn_are_you_sure_message(
    mut commands: Commands
){
    
}

fn listen_for_are_you_sure_message_requests(
    
){
    
}