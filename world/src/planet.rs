use {CubeMap, Surf, Vec3};

const _FACING_LAYOUT: &'static [&'static [i8; 3]] = &[
    &[ 0,  1,  0],  // up 0
    &[ 1,  0,  0],  // east 1
    &[ 0,  0,  1],  // south 2
    &[-1,  0,  0],  // west 3 
    &[ 0,  0, -1],  // north 4
    &[ 0, -1,  0],  // down 5
];

pub struct Planet {
    scale: usize,
    heights_map: CubeMap<f64>,
}

impl Planet {
    pub fn new(scale: usize) -> Self {
        if scale % 2 == 0 { panic!("Planet scale must be even"); }
        Planet {
            scale: scale,
            heights_map: CubeMap::new_hmg(scale, 1f64),
        }
    }

    pub fn warped(&self, side: u8, x: isize, y: isize) -> Vec3 {
        let s2 = (self.scale / 2) as isize;
        let s2f = (self.scale / 2) as f64;
        let sth = match side {
            0 => [x as f64, s2f, y as f64],
            1 => [s2f, x as f64, y as f64],
            2 => [x as f64, y as f64, s2f],
            3 => [-s2f, x as f64, y as f64],
            4 => [x as f64, y as f64, -s2f],
            5 => [x as f64, -s2f, y as f64],
            _ => panic!()
        };
        let w = (sth[0].powi(2) + sth[1].powi(2) + sth[2].powi(2)).sqrt();

        let h = *self.heights_map.get(side, (x as isize + s2) as usize, (y as isize + s2) as usize);
        [
            sth[0] / w * h,
            sth[1] / w * h, 
            sth[2] / w * h,
        ].into()
    }

    pub fn side(&self, idx: u8) -> &Surf<f64> {
        self.heights_map.side(idx)
    }

    pub fn side_mut(&mut self, idx: u8) -> &mut Surf<f64> {
        self.heights_map.side_mut(idx)
    }

    pub fn scale(&self) -> usize {
        self.scale
    }
}

#[cfg(test)]
mod tests {

    use super::Planet;

    #[test]
    fn warp1() {
        let planet = Planet::new(17);

        let p1 = planet.warped(0, 8, 8);
        let p2 = planet.warped(2, 8, 8);

        assert_eq!(format!("{:?}", p1), format!("{:?}", p2));
    }
}