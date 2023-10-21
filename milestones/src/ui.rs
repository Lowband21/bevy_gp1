use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Startup;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::render::view::Visibility;

use crate::GameState;

#[derive(Component)]
struct Menu;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu);
        app.add_systems(Startup, spawn_fps_text);
        app.add_systems(Update, (toggle_menu_visibility, resume_button));
        app.add_systems(
            Update,
            fps_display_system.run_if(in_state(GameState::Running)),
        );
    }
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    height: Val::Percent(60.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(20.0), // 100% - 60% = 40% / 2 = 20%
                    right: Val::Percent(20.0),
                    top: Val::Percent(20.0),
                    bottom: Val::Percent(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::GRAY.into(),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Menu,
        ))
        .with_children(|parent| {
            // Spawn a container for the Menu TextBundle
            let mut menu_container = parent.spawn((
                NodeBundle {
                    style: Style {
                        top: Val::Percent(35.0),
                        //left: Val::Percent(44.0),
                        width: Val::Percent(12.0),
                        height: Val::Percent(20.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Name::new("Menu Container"),
            ));

            // Spawn the "Menu" text inside the container
            menu_container.with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Menu",
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ));
            });

            // Spawn the button
            let mut button_entity = parent.spawn((
                ButtonBundle {
                    node: Node::default(),
                    button: Button::default(),
                    style: Style {
                        top: Val::Percent(50.0),
                        width: Val::Percent(20.0),
                        height: Val::Percent(8.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    interaction: Interaction::default(),
                    background_color: BackgroundColor(Color::rgb(0.1, 0.2, 0.3)),
                    border_color: BorderColor(Color::rgb(0.4, 0.5, 0.6)),
                    image: UiImage::default(),
                    transform: Transform::default(),
                    global_transform: GlobalTransform::default(),
                    visibility: Visibility::default(),
                    computed_visibility: ComputedVisibility::default(),
                    z_index: ZIndex::default(),
                    ..Default::default()
                },
                Name::new("Button"),
            ));

            // Add text to the button
            button_entity.with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Resume Game",
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ));
            });
        });
}

fn toggle_menu_visibility(
    mut app_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<Menu>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        let mut visibility = query.single_mut();
        match *visibility {
            Visibility::Visible => {
                *visibility = Visibility::Hidden;
                *app_state = State::new(GameState::Running);
                println!("Set Game State to Running");
            }
            Visibility::Hidden | Visibility::Inherited => {
                *visibility = Visibility::Visible;
                *app_state = State::new(GameState::Paused);
                println!("Set Game State to Paused");
            }
        }
    }
}

fn resume_button(
    button: Query<(&Button, &Interaction), With<Button>>,
    mut visibility: Query<&mut Visibility, With<Menu>>,
    mut app_state: ResMut<State<GameState>>,
) {
    for (_button, interaction) in button.iter() {
        match *interaction {
            Interaction::Pressed => {
                let mut visibility = visibility.single_mut();
                match *visibility {
                    Visibility::Visible => {
                        *visibility = Visibility::Hidden;
                        *app_state = State::new(GameState::Running); // Resume the game
                    }
                    Visibility::Hidden | Visibility::Inherited => {
                        *visibility = Visibility::Visible;
                        *app_state = State::new(GameState::Paused); // Pause the game
                    }
                }

                // Execute your desired code here
            }
            Interaction::Hovered => {
                // Optionally, handle the hover state if you want
            }
            Interaction::None => {
                // Optionally, handle the "None" state if you want
            }
        }
    }
}

// Don't forget to add the system to your AppBuilder when setting up your Bevy app
// app.add_system(button_system.system());
#[derive(Component)]
struct FpsText;

fn fps_display_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        for mut text in query.iter_mut() {
            if let Some(avg) = fps.average() {
                text.sections[0].value = format!("FPS: {}", avg);
            } else {
                text.sections[0].value = "FPS: N/A".to_string();
            }
        }
    }
}

pub fn spawn_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "FPS: 0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: TextAlignment::Right,
                linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
            },
            style: Style {
                position_type: PositionType::Absolute,
                margin: bevy::prelude::UiRect {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        FpsText,
    ));
}
