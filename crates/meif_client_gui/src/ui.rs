use fltk::{
    button::Button, frame::Frame, group::Row, input::Input, input::IntInput,
    prelude::*,
};

const DY: i32 = 16;
const DX: i32 = 16;

pub fn render() {
    let mut lbl = Frame::default()
        .with_pos(DX, DY)
        .with_size(128, 32)
        .with_label("Settings");
    lbl.set_frame(fltk::enums::FrameType::BorderBox);

    let mut row = Row::default()
        .with_size(256, 32)
        .with_pos(DX, DY + DY / 2 + 32);
    row.set_frame(fltk::enums::FrameType::BorderBox);

    let row_lbl = Frame::default()
        .with_pos(DX, DY + DY / 2 + 32)
        .with_size(128, 24)
        .with_label("Frequency, min: ");

    /* GLV: FIXME: Changing frequency */
    let _frec_edit = IntInput::default()
        .with_pos(DX + row_lbl.width() + DX, DY + DY / 2 + 32)
        .with_size(128, 24);

    row.end();

    let mut second_row = Row::default()
        .with_size(256, 32)
        .with_pos(DX, DY + row.y() + DY);
    second_row.set_frame(fltk::enums::FrameType::BorderBox);

    let row_lbl = Frame::default()
        .with_pos(DX, DY + DY / 2 + 32)
        .with_size(128, 24)
        .with_label("Frequency, min: ");

    /*GLV: FIXME: Changing path to storing files... */
    let _path_edit = Input::default()
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
    btn_001.set_callback(move |_| {});

    let mut btn_002 = Button::default()
        .with_pos(DX, btn_001.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 002");
    btn_002.set_callback(move |_| {});

    let mut btn_003 = Button::default()
        .with_pos(DX, btn_002.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 003");
    btn_003.set_callback(move |_| {});

    let mut btn_004 = Button::default()
        .with_pos(DX, btn_003.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 004");
    btn_004.set_callback(move |_| {});

    let mut imgs_synaptic_lbl = Frame::default()
        .with_pos(DX, btn_004.y() + DY + DY)
        .with_size(128, 32)
        .with_label("Synaptic data:");
    imgs_synaptic_lbl.set_frame(fltk::enums::FrameType::BorderBox);

    let mut syn_btn_001 = Button::default()
        .with_pos(DX, imgs_synaptic_lbl.y() + 2 * DY)
        .with_size(100, 32)
        .with_label("GIF 001");
    syn_btn_001.set_callback(move |_| {});
}
