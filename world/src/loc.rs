use Vec3;

pub struct Loc3 {
    pos: Vec3,
    vel: Vec3,
}

impl Loc3 {
    pub fn new(position: Vec3) -> Self {
        Loc3 { pos: position, vel: Vec3::zero() }
    }

    pub fn with_velocity(position: Vec3, velocity: Vec3) -> Self {
        Loc3 { pos: position, vel: velocity }
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn vel(&self) -> &Vec3 {
        &self.vel
    }
}