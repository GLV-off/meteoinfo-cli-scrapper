use std::error::Error;
use slint::Weak;
use slint::ComponentHandle;

use meif_client_ui_shared::MainWindow;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();

    let ui = MainWindow::new()
        .unwrap();

    // Слабая ссылка на окно через Weak<T>
    let ui_handle: Weak<MainWindow> = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui: MainWindow = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    let ui_handle: Weak<MainWindow> = ui.as_weak();
    ui.on_request_decrease_value(move || {
        let ui: MainWindow = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() - 1 );
    });


    ui.run().unwrap();
}