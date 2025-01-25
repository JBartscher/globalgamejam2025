use bevy::prelude::*;
use crate::{setup, GameState};

pub struct UiPlugin;
const MARGIN: Val = Val::Px(12.);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);


impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_menu)
            .add_systems(OnEnter(GameState::Game), cleanup_ui)
            .add_systems(Update, play_again_button_handler.run_if(in_state(GameState::GameOver)))
        ;
    }
}

fn play_again_button_handler(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Game)
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

/// remove all ui nodes
fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    commands
        .spawn((
                   Node {
                       width: Val::Percent(100.0),
                       height: Val::Percent(100.0),
                       align_items: AlignItems::Center,
                       justify_content: JustifyContent::Center,
                       ..default()
                   },
                    BackgroundColor(Color::BLACK),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Again?"),
                    TextFont {
                        font: asset_server.load("fonts/Kenney Mini Square.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
    }