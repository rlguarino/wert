use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Div<&f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: &f64) -> Vec3 {
        (1.0 / _rhs) * self
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Vec3 {
        self / &_rhs
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        self + &_rhs
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        &self + &_rhs
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        self - &_rhs
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        &self - &_rhs
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Mul<&f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: &f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<&Vec3> for &f64 {
    type Output = Vec3;
    fn mul(self, _rhs: &Vec3) -> Vec3 {
        _rhs * self
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, _rhs: &Vec3) -> Vec3 {
        &self * _rhs
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn add() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2 = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(
            &v1 + &v2,
            Vec3 {
                x: 2.0,
                y: 3.0,
                z: 4.0
            }
        );
    }
}
