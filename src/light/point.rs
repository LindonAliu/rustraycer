//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-lindon.aliu [SSH: 192.168.161.128]
// File description:
// point light
//

use crate::material::Color;
use crate::vector3d::{Point3D, Vector3D};
use crate::light::Light;
use crate::intersection::{Intersection};

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Point {
    pub pos: Point3D,
    pub color: Color
}

#[typetag::serde]
impl Light for Point {
    fn light(&self, intersection: &Intersection, color: &Color) -> Color {
        let l: Vector3D = self.pos - intersection.intersection_point;
        let cos_a: f64 = intersection.normal.dot(l) / (intersection.normal.length() * l.length());
        let multiplier: f64 = if cos_a > 0. {
            cos_a
        } else {
            0.
        };

        Color {
            r: ((self.color.r as f64 / 255.0) * multiplier * (color.r as f64)) as u8,
            g: ((self.color.g as f64 / 255.0) * multiplier * (color.g as f64)) as u8,
            b: ((self.color.b as f64 / 255.0) * multiplier * (color.b as f64)) as u8,
            a: color.a
        }
    }
}