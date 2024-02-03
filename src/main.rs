#![deny(unsafe_code)]

mod board;
use bevy::{prelude::*, window::EnabledButtons};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use board::*;

const TITLE: &str = "2D Chess!";
const RESOLUTION: f32 = 330.;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let board = Board::new();
    for piece in board.read().unwrap().get_pieces() {
        let color_txt = piece.get_color().to_string();
        let image = piece.get_kind().to_string();
        let texture = format!("{color_txt}_{image}.png");
        let piece_position = piece.get_position();
        let piece_x = (piece_position.y() as f32 - 3.5) * 40.0;
        let piece_y = (piece_position.x() as f32 - 3.5) * 40.0;
        let transform = Transform::from_xyz(piece_x, piece_y, 0.0);
        let sprite = SpriteBundle {
            texture: asset_server.load(texture),
            transform,
            ..default()
        };
        commands.spawn(sprite);
    }
}

fn main() {
    let _board = Board::new();
    App::new()
        .insert_resource(Msaa::Sample8)
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
                    resolution: (RESOLUTION, RESOLUTION).into(),
                    enabled_buttons: EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    resizable: false,
                    focused: true,
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, (setup_camera, setup_board))
        .run();
}
