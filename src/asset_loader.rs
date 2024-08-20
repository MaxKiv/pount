use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct AssetStore {
    pub card: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetStore>();
        app.add_systems(Startup, load_assets);
    }
}

fn load_assets(mut asset_store: ResMut<AssetStore>, asset_server: Res<AssetServer>) {
    *asset_store = AssetStore {
        card: asset_server.load("cube.glb#Scene0"),
    }
}
