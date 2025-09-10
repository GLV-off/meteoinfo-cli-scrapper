mod ui;

use fltk::{app, window::Window, prelude::* };

const MAIN_WINDOW_TITLE: &str = "Meteo Info Client";
const MAIN_WINDOW_WIDTH: i32 = 400;
const MAIN_WINDOW_HEIGHT: i32 = 500;
const MAIN_START_X: i32 = 100;
const MAIN_START_Y: i32 = 100;

fn main() -> Result<(), FltkError> {
    let app = app::App::default();

    let mut wind = Window::new(
        MAIN_START_X, MAIN_START_Y, 
        MAIN_WINDOW_WIDTH, MAIN_WINDOW_HEIGHT,
        MAIN_WINDOW_TITLE,
    );
    ui::render();
    wind.end();
    wind.show();

    app.run()
}
