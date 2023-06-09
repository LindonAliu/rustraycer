//
// EPITECH PROJECT, 2023
// cone
// File description:
// FreeKOSOVO
//

use crate::intersection::{Intersection, Ray};
use crate::material::Material;
use crate::shape::Shape;
use crate::vector3d::{Vector3D};
use serde::{Deserialize, Serialize};
use crate::shape::polynomial::{sq, intersect_polynomial};

#[derive(Serialize, Deserialize)]
pub struct Cone {
    pub angle: f64,
    pub material: Material,
}

fn cone_calcul_intersect(cone: &Cone, ray: &Ray, x: f64) -> Intersection {
    let intersection_point = ray.origin + ray.direction * x;
    let v = intersection_point.normalize();
    let local_center: Vector3D = Vector3D {
        x: 0.,
        y: 0.,
        z: intersection_point.z,
        w: 1.
    };
    let d = (local_center - intersection_point).normalize();
    let p = v.cross(d).normalize();
    let normal = p.cross(v);

    Intersection::new(intersection_point, normal, cone.material, ray)
}

#[typetag::serde]
impl Shape for Cone {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let zc = sq(self.angle.to_radians());
        let pt_cone: Vector3D = Vector3D {
            x: sq(ray.direction.x) + sq(ray.direction.y) - zc * sq(ray.direction.z),
            y: 2. * (ray.origin.x * ray.direction.x + ray.origin.y * ray.direction.y - zc * ray.origin.z * ray.direction.z),
            z: sq(ray.origin.x) + sq(ray.origin.y) - zc * sq(ray.origin.z),
            w: 1.,
        };

        intersect_polynomial(pt_cone).map(|x| cone_calcul_intersect(self, ray, x))
    }
}
