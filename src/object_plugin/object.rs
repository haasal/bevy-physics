use bevy::{ecs::system::Resource, prelude::*};
use bevy_prototype_lyon::{prelude::tess::geom::arrayvec::Array, shapes::Circle};

use crate::physics::{Dynamic, Dynamics, VerletObject2d};

use super::G;

#[derive(Component)]
pub struct Object {
    shape: Circle,
    dynamics: Dynamics,
    pub forces: Vec<Vec2>,
}

impl Object {
    pub fn new(shape: Circle, dynamics: Dynamics) -> Self {
        Self {
            shape,
            dynamics,
            forces: vec![Vec2::new(0.0, G)],
        }
    }
}

impl Dynamic for Object {
    fn r(&self) -> &Vec2 {
        &self.dynamics.r()
    }
    fn v(&self) -> &Vec2 {
        &self.dynamics.v()
    }
    fn a(&self) -> &Vec2 {
        &self.dynamics.a()
    }
    fn set_r(&mut self, r: Vec2) {
        self.dynamics.set_r(r);
    }
    fn set_v(&mut self, v: Vec2) {
        self.dynamics.set_v(v);
    }
    fn set_a(&mut self, a: Vec2) {
        self.dynamics.set_a(a);
    }
}

impl VerletObject2d for Object {
    fn ext_forces(&self) -> &[Vec2] {
        &self.forces
    }

    fn collision<O: crate::physics::Collider>(&self, _other: &O) -> Option<Vec2> {
        None
    }
}

pub fn move_obj(mut query: Query<(&mut Object, &mut Transform)>, time: Res<Time>) {
    for (mut o, mut t) in query.iter_mut() {
        o.update(time.delta_seconds(), &[]);
        t.translation = Vec3::new(o.r().x, o.r().y, 0.);
    }
}
