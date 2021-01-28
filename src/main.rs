mod snake_input;
mod snake_visuals;
mod snake_logic;

use bevy::prelude::*;
use bevy_rollback::*;
use bevy::core::FixedTimestep;
use snake_input::*;
use snake_visuals::*;
use snake_logic::*;

fn main() {

    let app = App::build()
        .add_plugin(SnakeVisualsPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(
            RollbackPlugin::with_buffer_size(60)
                .with_run_criteria(FixedTimestep::steps_per_second(60.0))
        )
        .add_plugin(SnakeInputPlugin)
        .add_plugin(SnakeLogic)
        .run();

}

