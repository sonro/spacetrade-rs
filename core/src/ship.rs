use crate::cargo::Cargo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    id: String,
    r#type: String,
    class: String,
    manufacturer: String,
    location: String,
    x: i32,
    y: i32,
    speed: u32,
    weapons: u32,
    plating: u32,
    max_cargo: u32,
    space_available: u32,
    cargo: Vec<Cargo>,
}
