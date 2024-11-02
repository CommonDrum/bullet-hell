use crate::game::prelude::*;
use bevy::color::palettes::basic::*;

#[derive(Component)]
struct OnMenuScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, button_system.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu); // Add cleanup system
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut next_app: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Start".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
                next_app.set(AppState::Game);
            }
            Interaction::Hovered => {
                text.sections[0].value = "New Game".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "New Game".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn((
        Camera2dBundle::default(),
        OnMenuScreen, // Tag for cleanup
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen, // Tag for cleanup
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    OnMenuScreen, // Tag for cleanup
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font: asset_server.load("fonts/JacquardaBastarda.ttf"),
                                font_size: 40.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        ),
                        OnMenuScreen, // Tag for cleanup
                    ));
                });
        });
}

// This function cleans up the menu entities on exiting the Menu state
fn cleanup_menu(mut commands: Commands, menu_entities: Query<Entity, With<OnMenuScreen>>) {
    for entity in &menu_entities {
        commands.entity(entity).despawn();
    }
}


// Testowanko
