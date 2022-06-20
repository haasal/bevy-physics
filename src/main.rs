mod object_plugin;
mod physics;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use object_plugin::ObjectPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjectPlugin)
        .add_plugin(ShapePlugin)
        .add_startup_system(startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
