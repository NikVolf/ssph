/// l1 x l2 surface artifacts
#[derive(Clone)]
pub struct Surf<T: Sized + Default + Clone> {
    l1: usize,
    l2: usize,
    heights: Vec<T>,
}

impl<T: Sized + Default + Clone> Surf<T> {
    pub fn new(l1: usize, l2: usize) -> Self {
        Surf { l1: l1, l2: l2, heights: vec![T::default(); l1 * l2] }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        self.heights.get(y * self.l2 + x).expect("Caller should provide existing location")
    }   

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.heights.get_mut(y * self.l2 + x).expect("Caller should provide existing location")
    }

    pub fn l1(&self) -> usize { self.l1 }

    pub fn l2(&self) -> usize { self.l2 }  
}

pub struct CubeMap<T: Sized + Default + Clone> {
    l1: usize,
    l2: usize,
    sides: [Surf<T>; 6],
}

impl<T: Sized + Default + Clone> CubeMap<T> {
    pub fn new(l1: usize, l2: usize) -> Self {
        let sides = [
            Surf::new(l1, l2),
            Surf::new(l1, l2),
            Surf::new(l1, l2),
            Surf::new(l1, l2),
            Surf::new(l1, l2),
            Surf::new(l1, l2),
        ];

        CubeMap {
            l1: l1,
            l2: l2,
            sides: sides,
        }
    }

    pub fn get(&self, side: u8, l1: usize, l2: usize) -> &T {
        self.sides[side as usize].get(l1, l2)
    }

    pub fn get_mut(&mut self, side: u8, l1: usize, l2: usize) -> &mut T {
        self.sides[side as usize].get_mut(l1, l2)
    }

    pub fn l1(&self) -> usize { self.l1 }

    pub fn l2(&self) -> usize { self.l2 }  
    
}

#[cfg(test)]
mod tests {

    use super::{Surf, CubeMap};

    #[test]
    fn new() {
        let mut map = Surf::new(5, 5);
        *map.get_mut(1, 2) = 15f64;

        assert!(*map.get(1, 2) > 14f64);
        assert!(*map.get(1, 2) < 16f64);
    }

    #[test]
    fn cube() {
        let mut cube_map = CubeMap::new(8, 8);
        *cube_map.get_mut(1, 4, 3) = 19f64;

        assert!(*cube_map.get(1, 4, 3) > 18f64);
        assert!(*cube_map.get(1, 4, 3) < 20f64);
    }
}