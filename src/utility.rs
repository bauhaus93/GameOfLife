use allegro::*;
use allegro_primitives::*;
use allegro_font::*;

pub trait Drawable {
    fn draw(&self, primitives: &PrimitivesAddon, boundary: (i32, i32));
}

pub fn initalize_allegro(width: i32, height: i32, update_frequency: i32) -> Result<(Core, Display, EventQueue, Timer, PrimitivesAddon, FontAddon, Font), String> {
    let mut core = match Core::init() {
        Ok(e) => e,
        Err(e) => return Err(String::from("Could not init core: ") + &e)
    };

    match core.install_keyboard() {
        Ok(e) => e,
        Err(_) => return Err(String::from("Could not install keyboard"))
    }

    let display = match Display::new(&core, width, height) {
        Ok(e) => e,
        Err(_) => return Err(String::from("Could not create display"))
    };

    let event_queue = match EventQueue::new(&core) {
        Ok(e) => e,
        Err(_) => return Err(String::from("Could not create event queue"))
    };

    let timer = match Timer::new(&core, 1.0 / update_frequency as f64) {
        Ok(e) => e,
        Err(_) => return Err(String::from("Could not create timer"))
    };

    let primitives_addon = match PrimitivesAddon::init(&core) {
        Ok(e) => e,
        Err(e) => return Err(String::from("Could not init primitives addon: ") + &e)
    };

    let font_addon = match FontAddon::init(&core) {
        Ok(e) => e,
        Err(e) => return Err(String::from("Could not init font addon: ") + &e)
    };

    let font = match Font::new_builtin(&font_addon) {
        Ok(e) => e,
        Err(_) => return Err(String::from("Could not load builtin font"))
    };

    event_queue.register_event_source(display.get_event_source());
    event_queue.register_event_source(core.get_keyboard_event_source());
    event_queue.register_event_source(timer.get_event_source());

    Ok((core, display, event_queue, timer, primitives_addon, font_addon, font))
}
