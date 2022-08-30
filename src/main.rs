use bevy::{
    prelude::*,
};

mod start_ui;
mod rail_me;

#[derive(Component)]
struct Person; 

#[derive(Component)]
struct Name(String); 

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("James".to_string()));
    commands.spawn().insert(Person).insert(Name("Matthew".to_string()));
    commands.spawn().insert(Person).insert(Name("Caitlyn".to_string()));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // do stuff
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(add_people)
        .add_system(greet_people);
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_plugin(start_ui::StartUI)
        .run();
}

