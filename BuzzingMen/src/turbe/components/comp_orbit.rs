use turbo::prelude::*;

use crate::turbe::transform;

use super::super::{component::Component, position::Position, transform::Transform, border::Border};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct OrbitComponent {
    pub orbit_position : Position,
    pub orbit_radius : f32,
    pub tock : f32
}

impl OrbitComponent {

    pub fn new(orbit_rad : f32 ) -> OrbitComponent {
        return OrbitComponent{
            orbit_position : Position { x: 0, y: 0, rotation: 0 },
            orbit_radius : orbit_rad,
            tock : 0.0};
    }

}

impl OrbitComponent {

    pub fn update(&mut self, transform : &mut Transform) {

        self.tock += 0.1 * (rand() % 3) as f32;
        if (self.tock > 360.0)
        {
            self.tock = 0.0;
        }

        transform.set_x(self.orbit_position.get_x() + (self.tock.sin() * self.orbit_radius) as i32);
        transform.set_y(self.orbit_position.get_y() + (self.tock.cos() * self.orbit_radius) as i32);

    }

    

}