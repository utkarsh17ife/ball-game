use bevy::prelude::*;

mod game;
mod main_menu;
mod events;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)       
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over_event)
        .run();
}
