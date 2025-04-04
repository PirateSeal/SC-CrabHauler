use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType {
    Planet,
    Moon,
    OrbitalStation,
    LagrangePoint,
    JumpPoint,
    Star,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub angle: f64,
    pub distance: f64,
    pub location_type: LocationType,
    pub position: Vec2,
}

impl Location {
    pub fn new(name: &str, angle: f64, distance: f64, location_type: LocationType) -> Self {
        // Convert polar coordinates to Cartesian
        let angle_rad = angle.to_radians();
        let position = Vec2::new(
            (distance * angle_rad.cos()) as f32,
            (distance * angle_rad.sin()) as f32,
        );

        Self {
            name: name.to_string(),
            angle,
            distance,
            location_type,
            position,
        }
    }

    pub fn distance_to(&self, other: &Location) -> f32 {
        self.position.distance(other.position)
    }
}
