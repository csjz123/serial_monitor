use serial;
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref EXAMPLE: u8 = 42;
}

fn main() {
    serial::ui_init();
}
