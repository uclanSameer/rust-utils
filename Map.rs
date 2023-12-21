struct Planet {
    waterHeight: i64,
    metalSpotLimit: i64,
    radius: i64,
    biome: String,
    temperature: i64,
    biomeScale: i64,
    heightRange: i64,
    metalClusters: i64,
    metalDensity: i64,
    mass: i64,
    waterDepth: i64,
}
struct Spec {
    position: Vec<f64>,
    type_info: String,
    material: String,
    height: f64,
    size: i64,
}
struct PlanetCSG {
    rotation: i64,
    spec: Spec,
}
struct Source {
    metal_spot_size: i64,
    metal_spots_density: f64,
    reclaim_metal_spot_size: i64,
    reclaim_metal_spots: i64,
    metal_spots: i64,
    reclaim_metal_spots_density: f64,
}
struct Info {
    planet: Planet,
    symmetrical: bool,
    planetCSG: Vec<PlanetCSG>,
    balanced: bool,
    source: Source,
    scale: i64,
}
struct Map {
    players: Vec<i64>,
    info: Info,
    name: String,
    version: String,
    creator: String,
    description: String,
}
