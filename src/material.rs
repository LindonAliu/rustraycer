//
// EPITECH PROJECT, 2023
// raytracer
// File description:
// material
//

use serde::{Deserialize, Serialize};

/// Contains the RGBA values of a color.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    #[serde(default = "default_a")]
    pub a: u8,
}

fn default_a() -> u8 {
    255
}

/// Contains the different materials that can be used.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Material {
    /// Represents a color, with optional transparency.
    Color(Color),
    /// Represents a mirror.
    Mirror,
}
