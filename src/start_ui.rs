use crate::consts::*;
use bevy::{
    prelude::*,
    winit::WinitSettings
};

pub struct StartUI;

impl Plugin for StartUI {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(system))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(enter))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(exit));
    }
}

const DEFAULT_FONT: &str = "fonts/Aboreto-Regular.ttf";
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const BUTTON_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);

fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(90.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                    "Start Game",
                    TextStyle {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 40.0,
                        color: BUTTON_TEXT.into(),
                    },
            ));
        });
}

fn system(mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>,
                mut  app_state: ResMut<State<AppState>>) {
    for (interaction, mut color, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                app_state.set(AppState::Game).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn exit(mut commands: Commands, mut entities: Query<Entity>) {
    for entity in &mut entities {
        commands.entity(entity).despawn();
    }
    
}
