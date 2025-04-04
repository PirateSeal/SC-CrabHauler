use sc_crabhauler::models::{GameData, LocationType};
use tracing::info;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("Starting Star Citizen Cargo Route Planner...");

    // Create game data
    let game_data = GameData::new();

    // Get all POIs
    let pois = game_data.get_all_pois();

    // Print all POIs
    info!("Star Citizen Universe contains {} POIs:", pois.len());
    for poi in pois {
        let type_name = match poi.location_type {
            LocationType::Planet => "Planet",
            LocationType::Moon => "Moon",
            LocationType::OrbitalStation => "Orbital Station",
            LocationType::LagrangePoint => "Lagrange Point",
            LocationType::JumpPoint => "Jump Point",
            LocationType::Star => "Star",
        };
        info!(
            "{} ({}) at position ({:.2}, {:.2})",
            poi.name, type_name, poi.position.x, poi.position.y
        );
    }

    // Get a specific location
    if let Some(location) = game_data.get_location_by_name("microTech") {
        info!(
            "Found microTech at position ({:.2}, {:.2})",
            location.position.x, location.position.y
        );
    }

    // Precompute distance map
    let _distance_map = game_data.precompute_distance_map();
    info!("Precomputed distance map for all POIs");

    // Example: Get distance between two locations
    if let (Some(loc1), Some(loc2)) = (
        game_data.get_location_by_name("Port Tressler"),
        game_data.get_location_by_name("Everus Harbour"),
    ) {
        let distance = loc1.distance_to(loc2);
        info!(
            "Distance from {} to {} is {:.2} Gm",
            loc1.name, loc2.name, distance
        );
    }
}
