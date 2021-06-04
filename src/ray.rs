use crate::math::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, distance: &f64) -> Vec3 {
        &self.origin + &(&self.direction * distance)
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::math::Vec3;

    #[test]
    fn point_at() {
        let ray = Ray {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: 1.0,
                y: 3.0,
                z: 2.0,
            },
        };
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 3.0,
                z: 2.0
            },
            ray.point_at(&1.0)
        );
        assert_eq!(
            Vec3 {
                x: 1.5,
                y: 4.5,
                z: 3.0
            },
            ray.point_at(&1.5)
        );
    }
}
