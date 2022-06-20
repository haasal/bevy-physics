mod object;
mod object_bundle;

use bevy::log::prelude::*;
use bevy::prelude::*;

use crate::object_plugin::object_bundle::ObjectBundle;

use self::object::move_obj;
pub struct ObjectPlugin;

const G: f32 = -9.81;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_objects).add_system(move_obj);
    }
}

fn make_objects(mut commands: Commands) {
    info!("Spawning objects");
    commands.spawn_bundle(ObjectBundle::default().with_v(Vec2::new(10.0, 5.0)));
    commands.spawn_bundle(
        ObjectBundle::default()
            .with_v(Vec2::new(50., 5.0))
            .with_r(Vec2::new(-200., 0.)),
    );
}
