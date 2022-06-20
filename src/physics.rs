use bevy::prelude::*;

// TODO: Implement physics as a plugin.
// That way I can loop over all colliders etc instead of doing it in the Object plugin

#[derive(Component, Default)]
pub struct Dynamics {
    pub r: Vec2,
    pub v: Vec2,
    pub a: Vec2,
}

pub trait Dynamic {
    /// Position Vector
    fn r(&self) -> &Vec2;
    /// Velocity Vector
    fn v(&self) -> &Vec2;
    /// Acceleration Vector
    fn a(&self) -> &Vec2;

    fn set_r(&mut self, r: Vec2);
    fn set_v(&mut self, v: Vec2);
    fn set_a(&mut self, a: Vec2);
}

impl Dynamic for Dynamics {
    fn r(&self) -> &Vec2 {
        &self.r
    }
    fn v(&self) -> &Vec2 {
        &self.v
    }
    fn a(&self) -> &Vec2 {
        &self.a
    }
    fn set_r(&mut self, r: Vec2) {
        self.r = r;
    }
    fn set_v(&mut self, v: Vec2) {
        self.v = v;
    }
    fn set_a(&mut self, a: Vec2) {
        self.a = a;
    }
}

/// Implement this to enable a verlet based physics simulation
pub trait VerletObject2d: Dynamic {
    /// All external forces like gravity or friction. These can be r, v and a dependant
    ///
    /// # Returns
    /// A vector of all external forces
    fn ext_forces(&self) -> &[Vec2];

    /// Calculate the next position of the object
    ///
    /// # Arguments
    /// * `dt`: The time step in seconds
    /// * `collisions`: A list of all collisions that happened during the last step
    fn update(&mut self, dt: f32, collisions: &[Collision]) {
        let new_r = *self.r() + *self.v() * dt + 0.5 * *self.a() * dt * dt;
        let new_a = self.apply_force(collisions); // currently no collisions -> no additional forces
        let new_v = *self.v() + 0.5 * (*self.a() + new_a) * dt;

        self.set_r(new_r);
        self.set_v(new_v);
        self.set_a(new_a);
    }

    /// Apply a force to the object as the combination of external + collision forces
    fn apply_force(&self, collisions: &[Collision]) -> Vec2 {
        let ext_force_sum = self.ext_forces().iter().sum::<Vec2>();
        // match self.collision::<_>() {
        //     Some(force) => force + ext_force_sum,
        //     None => ext_force_sum,
        // };
        ext_force_sum
    }

    /// Calculates behaviour on collisions with a collider.
    ///
    /// # Returns
    /// * A force vector to apply to the object.
    /// * `None` if collisions should be ignored or no collision happend
    fn collision<O: Collider>(&self, other: &O) -> Option<Vec2>;
}

pub trait Collider {}

pub struct Collision<'a> {
    one: &'a dyn Collider,
    other: &'a dyn Collider,
}
