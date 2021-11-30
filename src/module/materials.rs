use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MaterialDefinition {
    Color {
        r: u8,
        g: u8,
        b: u8,
    },
    Pbr {
        albedo: String,
        normal_map: String,
        occlusion: String,
        metallic_roughness_texture: String,
        emissive: String,
        roughness: f32,
        metallic: f32,
    },
}

pub fn default_pbr() -> crate::module::MaterialDefinition {
    MaterialDefinition::Pbr {
        albedo: String::new(),
        normal_map: String::new(),
        occlusion: String::new(),
        metallic_roughness_texture: String::new(),
        emissive: String::new(),
        roughness: 0.089,
        metallic: 0.0,
    }
}
