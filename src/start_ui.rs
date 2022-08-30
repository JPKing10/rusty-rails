
use bevy::{
    prelude::*,
    winit::WinitSettings
};

pub struct StartUI;

impl Plugin for StartUI {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app())
            .add_startup_system(setup)
            .add_system(button_system);
    }
}

const DEFAULT_FONT: &str = "fonts/Aboreto-Regular.ttf";
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);
const BUTTON_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn button_system(mut interaction_query: Query<(&Interaction, &mut UiColor, &Children), (Changed<Interaction>, With<Button>)>,
                 mut text_query: Query<&mut Text>, app: &mut App) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                app.add_plugin(rail_me::Level1);
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
