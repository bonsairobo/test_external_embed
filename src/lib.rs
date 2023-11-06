//! Example of loading an embedded asset.

use bevy::asset::embedded_asset;
use bevy::prelude::*;

pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        // Path to asset must be relative to this file, because that's how
        // include_bytes! works.
        embedded_asset!(app, "bevy_pixel_light.png");

        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("embedded://test_external_embed/bevy_pixel_light.png"),
        ..default()
    });
}
