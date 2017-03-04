use allegro_primitives::PrimitivesAddon;
use rand::distributions::{ Range, IndependentSample };
use rand;

use vector2::Vector2;
use utility::Drawable;
use rule::Rule;
use draw_data::{ DrawData, CellType };

pub struct CellularAutomata {
    size: (usize, usize),
    world: [Vector2<bool>; 2],
    foreground_world: usize,
    rule: Rule,
    ticks: u64,
}

impl Drawable for CellularAutomata {
    fn draw(&self, primitives: &PrimitivesAddon, draw_data: &DrawData) {


        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let curr_cell = self.get((x, y));

                if curr_cell == true {
                    match *draw_data.get_cell_type() {
                        CellType::Circle => {
                            let x = x as f32 * draw_data.get_cell_size() + draw_data.get_cell_size() / 2.0;
                            let y = y as f32 * draw_data.get_cell_size() + draw_data.get_cell_size() / 2.0;
                            primitives.draw_filled_circle(x, y, draw_data.get_cell_size() / 2.0, draw_data.get_cell_color());
                        },
                        CellType::Square => {
                            let x_1 = x as f32 * draw_data.get_cell_size();
                            let y_1 = y as f32 * draw_data.get_cell_size();
                            let x_2 = x_1 + draw_data.get_cell_size();
                            let y_2 = y_1 + draw_data.get_cell_size();
                            primitives.draw_filled_rectangle(x_1, y_1, x_2, y_2, draw_data.get_cell_color());
                        }
                    }

                }
            }
        }

    }
}

impl CellularAutomata {

    pub fn new(size: (usize, usize), rule_string: &str) -> Result<CellularAutomata, String> {

        let rule = match Rule::new(rule_string.to_string()) {
            Ok(r) => r,
            Err(s) => return Err(s)
        };


        let automata = CellularAutomata {
            size: size,
            world: [Vector2::new(false, size), Vector2::new(false, size)],
            foreground_world: 0,
            rule: rule,
            ticks: 0
        };

        Ok(automata)
    }

    pub fn get(&self, position: (usize, usize)) -> bool {
        self.world[self.foreground_world].get(position)
    }

    fn get_foreground(&self) -> &Vector2<bool> {
        &self.world[self.foreground_world]
    }

    fn get_background(&self) -> &Vector2<bool> {
        &self.world[1 - self.foreground_world]
    }

    fn get_foreground_mut(&mut self) -> &mut Vector2<bool> {
        &mut self.world[self.foreground_world]
    }

    fn get_background_mut(&mut self) -> &mut Vector2<bool> {
        &mut self.world[1 - self.foreground_world]
    }

    fn flip_worlds(&mut self) {
        self.foreground_world = 1 - self.foreground_world
    }

    pub fn get_rule_string(&self) -> &str {
        &self.rule.get_string()
    }

    /*pub fn count_neighbours(&self, position: (i32, i32)) -> u8 {
        static OFFSETS: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1),
                                           (-1, 0),           (1, 0),
                                           (-1, 1),  (0, 1),  (1, 1) ];
        let mut c = 0;
        for offset in &OFFSETS {
            let pos = (position.0 + offset.0, position.1 + offset.1);
            if self.in_boundaries(pos) &&  self.get_foreground().get((pos.0 as usize, pos.1 as usize)) == true  {
                c += 1;
            }
        }

        c
    }

    fn in_boundaries(&self, position: (i32, i32)) -> bool {
        position.0 >= 0 &&
        position.1 >= 0 &&
        position.0 < self.size.0 as i32 &&
        position.1 < self.size.1 as i32
    }*/

    pub fn count_neighbours(&self, position: (i32, i32)) -> u8 {
        static OFFSETS: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1),
                                           (-1, 0),           (1, 0),
                                           (-1, 1),  (0, 1),  (1, 1) ];
        let mut c = 0;
        for offset in &OFFSETS {
            let pos_x = match position.0 + offset.0 {
                e if e < 0 => self.size.0 as i32 + e,
                e if e >= self.size.0 as i32 => e - self.size.0 as i32,
                e => e
            };

            let pos_y = match position.1 + offset.1 {
                e if e < 0 => self.size.1 as i32 + e,
                e if e >= self.size.1 as i32 => e - self.size.1 as i32,
                e => e
            };

            if self.get_foreground().get((pos_x as usize, pos_y as usize)) == true  {
                c += 1;
            }
        }

        c
    }



    pub fn tick(&mut self) {


        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pos = (x, y);
                let curr_cell = self.get_foreground().get(pos);
                let neighbours = self.count_neighbours((x as i32, y as i32));

                if self.rule.apply(curr_cell, neighbours) {
                    *self.get_background_mut().get_mut(pos) = true;
                }
                else if self.get_background().get(pos) {
                    *self.get_background_mut().get_mut(pos) = false;
                }
            }
        }
        self.flip_worlds();

        self.ticks += 1;
    }

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub fn randomize(&mut self, percent: f32) {

        let range_x = Range::new(0, self.size.0);
        let range_y = Range::new(0, self.size.1);
        let mut rng = rand::thread_rng();

        let mut place_count = (self.size.0 as f32 * self.size.1 as f32 * percent) as u32;

        while place_count > 0 {
            let pos = (range_x.ind_sample(&mut rng), range_y.ind_sample(&mut rng));
            if self.get_foreground().get(pos) != true {
                *self.get_foreground_mut().get_mut(pos) = true;
                place_count -= 1;
            }
        }

    }

    #[allow(dead_code)]
    pub fn add_blinker(&mut self, position: (usize, usize)) {
        for i in 0..3 {
            let pos = (position.0 + i, position.1);
            *self.get_foreground_mut().get_mut(pos) = true;
        }
    }

    #[allow(dead_code)]
    pub fn add_r_pentomimo(&mut self, position: (usize, usize)) {
        *self.get_foreground_mut().get_mut((position.0 + 1, position.1)) = true;
        *self.get_foreground_mut().get_mut((position.0 + 2, position.1)) = true;
        *self.get_foreground_mut().get_mut((position.0, position.1 + 1)) = true;
        *self.get_foreground_mut().get_mut((position.0 + 1, position.1 + 1)) = true;
        *self.get_foreground_mut().get_mut((position.0 + 1, position.1 + 2)) = true;
    }
}
