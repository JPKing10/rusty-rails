use crate::consts::*;
use bevy::{
    prelude::*,
};

mod consts;
mod start_ui;
mod rail_me;

fn main() {
    App::new()
        .add_state(AppState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(start_ui::StartUI)
        .add_plugin(rail_me::Level1)
        .run();
}

