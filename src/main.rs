mod board;
use bevy::{prelude::*, window::EnabledButtons};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use board::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let board = Board::new();
    for piece in board.read().unwrap().get_pieces() {
        let color_txt: &str = match piece.get_color() {
            board::Color::White => "white",
            board::Color::Black => "black",
        };
        let image = match piece.get_kind() {
            Kind::King => "king",
            Kind::Queen => "queen",
            Kind::Bishop => "bishop",
            Kind::Knight => "knight",
            Kind::Pawn => "pawn",
            Kind::Rook => "rook",
        };
        let texture = format!("{color_txt}_{image}.png");
        let piece_position = piece.get_position();
        let piece_x = (piece_position.y() as f32 - 3.5) * 40.0;
        let piece_y = (piece_position.x() as f32 - 3.5) * 40.0;
        let transform = Transform::from_xyz(piece_x, piece_y, 0.0);
        let sprite = SpriteBundle {
            texture: asset_server.load(texture),
            transform: transform,
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
                    title: "2D Chess!".into(),
                    resolution: (330., 330.).into(),
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
