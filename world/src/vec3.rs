use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Self {
        Vec3 { x: 0f64, y: 0f64, z: 0f64}
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl From<[f64; 3]> for Vec3 {
    fn from(v: [f64; 3]) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

#[cfg(test)]
mod tests {

    use super::Vec3;

    #[test]
    fn adds() {
        let p1 = Vec3::new(10f64, 12f64, 14f64);
        let p2 = Vec3::new(10f64, 18f64, 16f64);

        let p3 = p1 + p2;

        assert!(p3.x > 19f64);
        assert!(p3.x < 21f64);
        assert!(p3.y > 29f64);
        assert!(p3.y < 31f64);
        assert!(p3.z > 29f64);
        assert!(p3.z < 31f64);
    }

    #[test]
    fn subs() {
        let p1 = Vec3::new(10f64, 12f64, 14f64);
        let p2 = Vec3::new(10f64, 18f64, 16f64);

        let p3 = p1 - p2;

        assert!(p3.x > -1f64);
        assert!(p3.x < 1f64);
        assert!(p3.y > -7f64);
        assert!(p3.y < -5f64);
        assert!(p3.z > -3f64);
        assert!(p3.z < -1f64);
    }    
}
