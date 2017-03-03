
pub struct Vector2<T> {
    size: (usize, usize),
    data: Vec<T>
}

impl<T> Vector2<T>
    where T: Clone + Copy {

    pub fn new(init_val: T, size: (usize, usize)) -> Vector2<T> {
        Vector2{
            size: size,
            data: vec![init_val; size.0 * size.1]
        }
    }

    fn position_to_index(&self, position: (usize, usize)) -> usize {
        position.1 * self.size.0 + position.0
    }

    pub fn get_mut(&mut self, position: (usize, usize)) -> &mut T {
        let index = self.position_to_index(position);
        &mut self.data[index]
    }


    pub fn get(&self, position: (usize, usize)) -> T {
        self.data[self.position_to_index(position)]
    }

    pub fn clear(&mut self, value: T) {
        for e in &mut self.data {
            *e = value;
        }
    }
}
