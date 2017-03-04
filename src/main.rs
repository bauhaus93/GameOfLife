extern crate allegro;
extern crate allegro_primitives;
extern crate allegro_font;
extern crate rand;

mod cellular_automata;
mod vector2;
mod utility;
mod rule;
mod draw_data;

use std::time::{Duration, Instant};

use allegro::Color;
use allegro_font::{ FontAlign, FontDrawing };

use cellular_automata::CellularAutomata;
use utility::{ initalize_allegro, Drawable };
use draw_data::{ DrawData, CellType };

/*
    Cool Stuff:
        x/28 rpentomimo where x = {5, 6, 7, 8}
        126/234589 rpentomimo
        /289 rpentomimo (like x/28)

*/

fn main() {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;
    const FRAME_RATE: u32 = 60;
    const CELL_COUNT_X: u32 = WIDTH / 6;
    const CELL_COUNT_Y: u32 = HEIGHT / 6;
    const CELL_COLOR: (u8, u8, u8) = (0xFF, 0xFF, 0);
    const CELL_TYPE: CellType = CellType::Square;
    const RULE: &'static str = "23/3";

    let result = initalize_allegro(WIDTH as i32, HEIGHT as i32, FRAME_RATE as i32);
    let (core, display, event_queue, timer, primitives_addon, font_addon, font) = match result {
        Ok(e) => e,
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };

    let mut automata = match CellularAutomata::new((CELL_COUNT_X as usize, CELL_COUNT_Y as usize), RULE) {
        Ok(a) => a,
        Err(s) => {
            println!("Could not create automata: {}", s);
            return;
        }
    };

    let draw_data = DrawData::new((CELL_COUNT_X, CELL_COUNT_Y), (WIDTH, HEIGHT), CELL_COLOR, CELL_TYPE);

    //automata.add_blinker((10, 10));
    automata.add_r_pentomimo((CELL_COUNT_X as usize / 2, CELL_COUNT_Y as usize / 2));
    //automata.randomize(0.3);

    let mut redraw = true;
    let black = Color::from_rgb(0, 0, 0);
    let white = Color::from_rgb(0xFF, 0xFF, 0xFF);

    'exit: loop {

        if redraw && event_queue.is_empty() {
            core.clear_to_color(black);
            automata.draw(&primitives_addon, &draw_data);
            core.draw_text(&font, white, 5.0, 5.0, FontAlign::Left, &format!("ticks:   {}", automata.get_ticks()));
            core.draw_text(&font, white, 5.0, 15.0, FontAlign::Left, &format!("ruleset: {}", automata.get_rule_string()));
            core.flip_display();
            redraw = false;
        }

        match event_queue.wait_for_event() {

            allegro::KeyDown{keycode: k, ..} =>
                match k {
                    allegro::KeyCode::Escape => break 'exit,
                    allegro::KeyCode::Space => {
                        match timer.is_started() {
                            true => timer.stop(),
                            false => timer.start()
                        }
                    },
                    allegro::KeyCode::F => {
                        timer.stop();
                        for _ in 0..100 {
                            automata.tick();
                        }
                        timer.start();
                    },
                    _ => {}
            },


            allegro::TimerTick{..} => {
                automata.tick();
                redraw = true;
            },
            _ => {}

        }
    }
    timer.stop();




}
