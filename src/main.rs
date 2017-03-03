extern crate allegro;
extern crate allegro_primitives;
extern crate allegro_font;
extern crate rand;

mod cellular_automata;
mod vector2;
mod utility;
mod rules;

use allegro::Color;
use allegro_font::{ FontAlign, FontDrawing };

use cellular_automata::CellularAutomata;
use utility::{ initalize_allegro, Drawable };
use rules::{ rule_1, rule_2, rule_3, rule_4 };


fn main() {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 800;

    let result = initalize_allegro(WIDTH, HEIGHT, 20);
    let (core, display, event_queue, timer, primitives_addon, font_addon, font) = match result {
        Ok(e) => e,
        Err(e) => {
            println!("ERROR: {}", e);
            return;
        }
    };

    let mut automata = CellularAutomata::new((300, 300));
    automata.add_rule(rule_1);
    automata.add_rule(rule_2);
    automata.add_rule(rule_3);
    automata.add_rule(rule_4);

    //automata.add_blinker((5, 5));
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
            core.draw_text(&font, white, 5.0, 5.0, FontAlign::Left, &format!("ticks: {}", automata.get_ticks()));
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
                        for _ in 0..100 {
                            automata.tick();
                        }
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
