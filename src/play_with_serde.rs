use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDetail {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: i32,
}

pub fn json_plus_std_file_example() {
    // read test.json file
    let json = std::fs::read_to_string("test.json").unwrap();

    // read using serde_json
    let mut test: Test = serde_json::from_str(&json).unwrap();

    test.name = "Spiral 4v4 NS NEW".to_string();

    println!("test: {:?}", &test.info.source);

    let test_json = serde_json::to_string(&test).unwrap();


    // write to file
    std::fs::write("test.json", &test_json).unwrap();
}

#[derive(Serialize, Deserialize)]
struct Planet {
    waterHeight: i64,
    metalClusters: i64,
    biome: String,
    biomeScale: i64,
    metalDensity: i64,
    radius: i64,
    temperature: i64,
    metalSpotLimit: i64,
    waterDepth: i64,
    mass: i64,
    heightRange: i64,
}

#[derive(Serialize, Deserialize)]
struct Spec {
    height: f64,
    size: i64,
    material: String,
    position: Vec<f64>,
    type_info: String,
}

#[derive(Serialize, Deserialize)]
struct PlanetCSG {
    rotation: i64,
    spec: Spec,
}

#[derive(Serialize, Deserialize, Debug)]
struct Source {
    metal_spot_size: i64,
    reclaim_metal_spots: i64,
    reclaim_metal_spot_size: i64,
    metal_spots_density: f64,
    reclaim_metal_spots_density: f64,
    metal_spots: i64,
}

#[derive(Serialize, Deserialize)]
struct Info {
    planetCSG: Vec<PlanetCSG>,
    planet: Planet,
    source: Source,
    balanced: bool,
    scale: i64,
    symmetrical: bool,
}
#[derive(Serialize, Deserialize)]
struct Test {
    creator: String,
    description: String,
    players: Vec<i64>,
    name: String,
    info: Info,
    version: String,
}
