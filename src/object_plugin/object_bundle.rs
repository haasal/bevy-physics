use crate::physics::{Dynamic, Dynamics};

use super::object::Object;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

#[derive(Bundle)]
pub struct ObjectBundle {
    #[bundle]
    bundle: ShapeBundle,
    object: Object,
}

impl Default for ObjectBundle {
    fn default() -> Self {
        let shape = shapes::Circle {
            radius: 50.,
            ..default()
        };

        Self {
            bundle: GeometryBuilder::build_as(
                &shape,
                DrawMode::Fill(FillMode::color(Color::WHITE)),
                Transform::default(),
            ),
            object: Object::new(shape, Dynamics::default()),
        }
    }
}

impl ObjectBundle {
    pub fn with_v(mut self, v: Vec2) -> Self {
        self.object.set_v(v);
        self
    }

    pub fn with_r(mut self, r: Vec2) -> Self {
        self.object.set_r(r);
        self
    }
}
