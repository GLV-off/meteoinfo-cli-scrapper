use fltk::{app, button::Button, frame::Frame, group::Row, prelude::*, text::TextEditor, window::Window};

const MAIN_WINDOW_TITLE: &'static str  = "Meteo Info Client";
const MAIN_WINDOW_WIDTH: i32 = 400;
const MAIN_WINDOW_HEIGHT: i32 = 500;

const DY: i32 = 16;
const DX: i32 = 16;

fn main() -> Result<(), FltkError> {
    let app = app::App::default();
 
    let mut wind = Window::new(
        100, 100, 
        MAIN_WINDOW_WIDTH, 
        MAIN_WINDOW_HEIGHT, 
        MAIN_WINDOW_TITLE
    );

    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;

    let mut lbl = Frame::default()
        .with_pos(DX, DY)
        .with_size(128, 32)
        .with_label("Settings");
    lbl.set_frame(fltk::enums::FrameType::BorderBox);

    let mut row = Row::default()
        .with_size(256, 24)
        .with_pos(DX, DY + DY / 2 + 32);
    row.set_frame(fltk::enums::FrameType::BorderBox);

    let mut row_lbl = Frame::default()
        .with_pos(DX, DY + DY / 2 + 32)
        .with_size(128, 24)
        .with_label("Frequency, min: ");
    // row_lbl.set_frame(fltk::enums::FrameType::BorderBox);

    let mut row_edit = TextEditor::default()
        .with_pos(DX + row_lbl.width() + DX, DY + DY / 2 + 32)
        .with_size(128, 24);

    row.end();

    let mut second_row = Row::default()
        .with_size(256, 24)
        .with_pos(DX, DY + row.y() + DY);
    second_row.set_frame(fltk::enums::FrameType::BorderBox);

    let mut row_lbl = Frame::default()
        .with_pos(DX, DY + DY / 2 + 32)
        .with_size(128, 24)
        .with_label("Frequency, min: ");
    // row_lbl.set_frame(fltk::enums::FrameType::BorderBox);

    let mut row_edit = TextEditor::default()
        .with_pos(DX + row_lbl.width() + DX, DY + DY / 2 + 32)
        .with_size(128, 24);

    second_row.end();

    let mut fact_data_lbl = Frame::default()
        .with_pos(DX, second_row.y() + DY + DY)
        .with_size(128, 32)
        .with_label("Fact data:");
    fact_data_lbl.set_frame(fltk::enums::FrameType::BorderBox);

    let mut imgs_geostat_lbl = Frame::default()
        .with_pos(DX, fact_data_lbl.y() + DY + DY)
        .with_size(128, 32)
        .with_label("Geostat imgs:");
    imgs_geostat_lbl.set_frame(fltk::enums::FrameType::BorderBox);

    let mut btn_001 = Button::default()
        .with_pos(DX, imgs_geostat_lbl.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 001");
    btn_001.set_callback(move |local_btn|{
        
    });

    let mut btn_002 = Button::default()
        .with_pos(DX, btn_001.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 002");
    btn_002.set_callback(move |local_btn|{
        
    });

    let mut btn_003 = Button::default()
        .with_pos(DX, btn_002.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 003");
    btn_003.set_callback(move |local_btn|{
        
    });

    let mut btn_004 = Button::default()
        .with_pos(DX, btn_003.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 004");
    btn_004.set_callback(move |local_btn|{
        
    });

    let mut imgs_synaptic_lbl = Frame::default()
        .with_pos(DX, btn_004.y() + DY + DY)
        .with_size(128, 32)
        .with_label("Synaptic data:");
    imgs_synaptic_lbl.set_frame(fltk::enums::FrameType::BorderBox);

    let mut syn_btn_001 = Button::default()
        .with_pos(DX, imgs_synaptic_lbl.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 001");
    syn_btn_001.set_callback(move |local_btn|{
        
    });
    
    wind.end();   
    wind.show();

    app.run()
}
