use allegro::Color;
use allegro_primitives::PrimitivesAddon;
use rand::distributions::{ Range, IndependentSample };
use rand;

use vector2::Vector2;
use utility::Drawable;
use rule::Rule;

pub struct CellularAutomata {
    size: (usize, usize),
    world: Vector2<bool>,
    rule: Rule,
    ticks: u64
}


impl Drawable for CellularAutomata {
    fn draw(&self, primitives: &PrimitivesAddon, boundary: (i32, i32)) {

        //let white = Color::from_rgb(0xFF, 0xFF, 0xFF);
        let cell_color = Color::from_rgb(0xFF, 0, 0);

        let d_x = boundary.0 as f32 / self.size.0 as f32;
        let d_y = boundary.1 as f32 / self.size.1 as f32;

        let diameter = match d_x < d_y {
            true => d_x,
            false => d_y
        };

        let radius = diameter / 2.0;

        /*for x in 0..self.size.0 {
            primitives.draw_line(x as f32 * diameter, 0.0, x as f32 * diameter, diameter * self.size.1 as f32, white, 1.0);
        }

        for y in 0..self.size.1 {
            primitives.draw_line(0.0, y as f32 * diameter, diameter * self.size.0 as f32, y as f32 * diameter, white, 1.0);
        }*/

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let curr_cell = self.get((x, y));

                if curr_cell == true {
                    primitives.draw_filled_circle(x as f32 * diameter + radius, y as f32 * diameter + radius, radius, cell_color);
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
            world: Vector2::new(false, size),
            rule: rule,
            ticks: 0
        };

        Ok(automata)
    }

    pub fn get(&self, position: (usize, usize)) -> bool {
        self.world.get(position)
    }

    pub fn get_rule_string(&self) -> &str {
        &self.rule.get_string()
    }

    pub fn count_neighbours(&self, position: (i32, i32)) -> u8 {
        static OFFSETS: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1),
                                           (-1, 0),           (1, 0),
                                           (-1, 1),  (0, 1),  (1, 1) ];
        let mut c = 0;
        for offset in &OFFSETS {
            let pos = (position.0 + offset.0, position.1 + offset.1);
            if self.in_boundaries(pos) &&  self.get((pos.0 as usize, pos.1 as usize)) == true  {
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
    }

    pub fn tick(&mut self) {
        let mut next_world = Vector2::new(false, self.size);

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let curr_cell = self.get((x, y));
                let neighbours = self.count_neighbours((x as i32, y as i32));

                if self.rule.apply(curr_cell, neighbours) {
                    *next_world.get_mut((x, y)) = true;
                }
            }
        }
        self.world = next_world;
        self.ticks += 1;
    }

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub fn randomize(&mut self, percent: f32) {
        static MAX_TRIES: u32 = 10;

        let place_count = (self.size.0 as f32 * self.size.1 as f32 * percent) as u32;

        let range_x = Range::new(0, self.size.0);
        let range_y = Range::new(0, self.size.1);
        let mut rng = rand::thread_rng();


        for _ in 0..place_count {
            for _ in 0..MAX_TRIES {
                let pos = (range_x.ind_sample(&mut rng), range_y.ind_sample(&mut rng));
                if self.world.get(pos) != true {
                    *self.world.get_mut(pos) = true;
                    break;
                }
            }
        }

    }

    #[allow(dead_code)]
    pub fn add_blinker(&mut self, position: (usize, usize)) {
        for i in 0..3 {
            let pos = (position.0 + i, position.1);
            *self.world.get_mut(pos) = true;
        }
    }

    #[allow(dead_code)]
    pub fn add_r_pentomimo(&mut self, position: (usize, usize)) {
        *self.world.get_mut((position.0 + 1, position.1)) = true;
        *self.world.get_mut((position.0 + 2, position.1)) = true;
        *self.world.get_mut((position.0, position.1 + 1)) = true;
        *self.world.get_mut((position.0 + 1, position.1 + 1)) = true;
        *self.world.get_mut((position.0 + 1, position.1 + 2)) = true;
    }
}
