mod components;
use components::ui::SerialTool;
pub fn ui_init() {
    let serial = SerialTool::default();
    serial.init();
}
