use bevy::prelude::*;
use crate::{types::GameState};
pub struct GameOverPlugin;

#[derive(Component, Default, Clone)]
pub struct OnlyInGameOver;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(spawn_ui))
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(load_game))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup));
    }
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        }
    ).insert(OnlyInGameOver)

        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                image: asset_server.load("gameoverscreen.png").into(),
                ..Default::default()
            });
        }).insert(OnlyInGameOver);
}
fn load_game(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keyboard_input.just_released(KeyCode::Space) || keyboard_input.just_released(KeyCode::Return)
    {
        game_state.set(GameState::InGame).unwrap();
    }

    if keyboard_input.just_released(KeyCode::Back)
    {
        game_state.set(GameState::MainMenu).unwrap();
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<OnlyInGameOver>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
