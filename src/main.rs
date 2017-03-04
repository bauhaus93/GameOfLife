extern crate allegro;
extern crate allegro_primitives;
extern crate allegro_font;
extern crate rand;

mod cellular_automata;
mod vector2;
mod utility;
mod rule;

use allegro::Color;
use allegro_font::{ FontAlign, FontDrawing };

use cellular_automata::CellularAutomata;
use utility::{ initalize_allegro, Drawable };
use rule::Rule;

fn main() {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 800;

    let result = initalize_allegro(WIDTH, HEIGHT, 15);
    let (core, display, event_queue, timer, primitives_addon, font_addon, font) = match result {
        Ok(e) => e,
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };

    let mut automata = match CellularAutomata::new((200, 200), "23/3") {
        Ok(a) => a,
        Err(s) => {
            println!("Could not create automata: {}", s);
            return;
        }
    };

    //automata.add_blinker((10, 10));
    //automata.add_r_pentomimo((50, 50));
    automata.randomize(0.3);

    let mut redraw = true;
    let black = Color::from_rgb(0, 0, 0);
    let white = Color::from_rgb(0xFF, 0xFF, 0xFF);
    //timer.start();
    'exit: loop {

        if redraw && event_queue.is_empty() {
            core.clear_to_color(black);
            automata.draw(&primitives_addon, (WIDTH, HEIGHT));
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
