//! Kani user defined types
use serde::{Deserialize, Serialize};

/// Workspace Config (from `Kani.toml`)
#[derive(Deserialize, Serialize)]
pub struct KaniConfig {
    pub name: String,
    pub manager: Option<String>,
    pub stages: Vec<String>,
    pub boards: Vec<KaniBoard>,
    pub archive: Vec<KaniCard>,
}

/// Workspace Kani Board Struct (from `.kani` files)
#[derive(Deserialize, Serialize)]
pub struct KaniBoard {
    pub name: String,
    pub cards: Vec<KaniCard>,
}

/// Board Kani Card
#[derive(Deserialize, Serialize)]
pub struct KaniCard {
    pub name: String,
    pub stage: String,
    pub owners: Vec<KaniMember>,
}

/// Kani Card Members
#[derive(Deserialize, Serialize)]
pub struct KaniMember {
    pub name: String,
    pub email: Option<String>,
    pub role: Option<String>,
}
