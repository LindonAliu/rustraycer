//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// light
//

mod point;
mod ambient;
mod directional;

pub use point::Point;
pub use ambient::Ambient;
pub use directional::Directional;

use crate::material::Color;
use crate::intersection::{Intersection};
use crate::shape::Shape;

#[typetag::serde(tag = "type")]
pub trait Light {
    /// Returns the color of the light at the intersection point
    /// Will be black if the light is not visible from the intersection point
    fn light(&self, intersection: &Intersection, color: &Color, shape: &dyn Shape) -> Color;
}
