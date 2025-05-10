mod ui;

use fltk::{app, prelude::*, window::Window};

const MAIN_WINDOW_TITLE: &str = "Meteo Info Client";
const MAIN_WINDOW_WIDTH: i32 = 400;
const MAIN_WINDOW_HEIGHT: i32 = 500;

fn main() -> Result<(), FltkError> {
    let app = app::App::default();

    let mut wind = Window::new(
        100,
        100,
        MAIN_WINDOW_WIDTH,
        MAIN_WINDOW_HEIGHT,
        MAIN_WINDOW_TITLE,
    );

    ui::render();

    wind.end();
    wind.show();

    app.run()
}
