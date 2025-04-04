use super::location::Location;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    pub name: String,
    pub lagrange_color: String,
    pub planet: Location,
    pub orbital_station: Location,
    pub lagrange_points: Vec<Location>,
    pub moons: Vec<Location>,
}