use super::location::{Location, LocationType};
use super::star_system::StarSystem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub systems: Vec<StarSystem>,
    pub jump_points: Vec<Location>,
    pub central_star: Location,
}

impl GameData {
    pub fn new() -> Self {
        // Create central star
        let central_star = Location::new("Central Star", 0.0, 0.0, LocationType::Star);

        // Create systems
        let systems = vec![
            // Hurston system
            StarSystem {
                name: "Hurston".to_string(),
                lagrange_color: "#FF69B4".to_string(), // Hot pink
                planet: Location::new("Hurston", 0.0, 12.85, LocationType::Planet),
                orbital_station: Location::new(
                    "Everus Harbour",
                    0.0,
                    12.850459,
                    LocationType::OrbitalStation,
                ),
                lagrange_points: vec![
                    Location::new("HUR-L1", 0.0, 11.56, LocationType::LagrangePoint),
                    Location::new("HUR-L2", 0.0, 14.13, LocationType::LagrangePoint),
                    Location::new("HUR-L3", -179.99, 12.85, LocationType::LagrangePoint),
                    Location::new("HUR-L4", 60.0, 12.85, LocationType::LagrangePoint),
                    Location::new("HUR-L5", -60.0, 12.85, LocationType::LagrangePoint),
                ],
                moons: vec![
                    Location::new("Arial", -0.13, 12.89, LocationType::Moon),
                    Location::new("Aberdeen", 0.18, 12.9, LocationType::Moon),
                    Location::new("Magda", -0.33, 12.79, LocationType::Moon),
                    Location::new("Ita", 0.51, 12.83, LocationType::Moon),
                ],
            },
            // Crusader system
            StarSystem {
                name: "Crusader".to_string(),
                lagrange_color: "#00CED1".to_string(), // Dark turquoise
                planet: Location::new("Crusader", -171.99, 19.14, LocationType::Planet),
                orbital_station: Location::new(
                    "Seraphim Station",
                    -171.98,
                    19.15,
                    LocationType::OrbitalStation,
                ),
                lagrange_points: vec![
                    Location::new("CRU-L1", -171.99, 17.23, LocationType::LagrangePoint),
                    Location::new("CRU-L2", -171.99, 21.06, LocationType::LagrangePoint),
                    Location::new("CRU-L3", 8.0, 19.14, LocationType::LagrangePoint),
                    Location::new("CRU-L4", -112.0, 19.14, LocationType::LagrangePoint),
                    Location::new("CRU-L5", 127.99, 19.14, LocationType::LagrangePoint),
                ],
                moons: vec![
                    Location::new("Cellin", -171.88, 19.17, LocationType::Moon),
                    Location::new("Daymar", -172.14, 19.1, LocationType::Moon),
                    Location::new("Yela", -172.17, 19.2, LocationType::Moon),
                ],
            },
            // ArcCorp system
            StarSystem {
                name: "ArcCorp".to_string(),
                lagrange_color: "#FFD700".to_string(), // Gold
                planet: Location::new("ArcCorp", -50.0, 28.91, LocationType::Planet),
                orbital_station: Location::new(
                    "Baijini Point",
                    -50.0,
                    28.910459,
                    LocationType::OrbitalStation,
                ),
                lagrange_points: vec![
                    Location::new("ARC-L1", -49.99, 26.09, LocationType::LagrangePoint),
                    Location::new("ARC-L2", -49.99, 31.8, LocationType::LagrangePoint),
                    Location::new("ARC-L3", 149.99, 28.91, LocationType::LagrangePoint),
                    Location::new("ARC-L4", -9.99, 28.91, LocationType::LagrangePoint),
                    Location::new("ARC-L5", -109.99, 28.91, LocationType::LagrangePoint),
                ],
                moons: vec![
                    Location::new("Lyria", -49.78, 28.96, LocationType::Moon),
                    Location::new("Wala", -50.12, 28.66, LocationType::Moon),
                ],
            },
            // microTech system
            StarSystem {
                name: "microTech".to_string(),
                lagrange_color: "#8A2BE2".to_string(), // Blue violet
                planet: Location::new("microTech", 58.86, 43.44, LocationType::Planet),
                orbital_station: Location::new(
                    "Port Tressler",
                    58.86,
                    43.440459,
                    LocationType::OrbitalStation,
                ),
                lagrange_points: vec![
                    Location::new("MIC-L1", 58.86, 39.9, LocationType::LagrangePoint),
                    Location::new("MIC-L2", 58.86, 47.78, LocationType::LagrangePoint),
                    Location::new("MIC-L3", -121.12, 43.44, LocationType::LagrangePoint),
                    Location::new("MIC-L4", 118.86, 43.44, LocationType::LagrangePoint),
                    Location::new("MIC-L5", -1.12, 43.44, LocationType::LagrangePoint),
                ],
                moons: vec![
                    Location::new("Calliope", 58.92, 43.39, LocationType::Moon),
                    Location::new("Clio", 58.78, 43.36, LocationType::Moon),
                    Location::new("Euterpe", 58.76, 43.37, LocationType::Moon),
                ],
            },
        ];

        // Create jump points
        let jump_points = vec![
            Location::new("Stanton-Pyro", -83.25, 28.3, LocationType::JumpPoint),
            Location::new("Stanton-Terra", -5.88, 51.57, LocationType::JumpPoint),
            Location::new("Stanton-Magnus", 159.35, 69.55, LocationType::JumpPoint),
        ];

        Self {
            systems,
            jump_points,
            central_star,
        }
    }

    // Helper method to get all POIs (Points of Interest)
    pub fn get_all_pois(&self) -> Vec<&Location> {
        let mut pois = Vec::new();

        pois.push(&self.central_star);

        for system in &self.systems {
            pois.push(&system.planet);
            pois.push(&system.orbital_station);

            for lagrange_point in &system.lagrange_points {
                pois.push(lagrange_point);
            }

            for moon in &system.moons {
                pois.push(moon);
            }
        }

        for jump_point in &self.jump_points {
            pois.push(jump_point);
        }

        pois
    }

    // Helper method to get a location by name
    pub fn get_location_by_name(&self, name: &str) -> Option<&Location> {
        self.get_all_pois().into_iter().find(|loc| loc.name == name)
    }

    // Precompute distance map for all POIs
    pub fn precompute_distance_map(
        &self,
    ) -> std::collections::HashMap<String, std::collections::HashMap<String, f32>> {
        use std::collections::HashMap;
        let pois = self.get_all_pois();
        let mut distance_map = HashMap::new();

        for poi1 in pois.iter() {
            let mut inner_map = HashMap::new();
            for poi2 in pois.iter() {
                inner_map.insert(poi2.name.clone(), poi1.distance_to(poi2));
            }
            distance_map.insert(poi1.name.clone(), inner_map);
        }

        distance_map
    }
}
