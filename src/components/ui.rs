extern crate native_windows_gui as nwg;
use crate::components::serial_port;
use nwg::NativeUi;
use serialport::SerialPort;

use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    static ref ARRAY: Mutex<Vec<bool>> = Mutex::new(vec![false]);
    static ref PORT: Mutex<Vec<Option<Box<dyn SerialPort>>>> = Mutex::new(vec![None]);
}

#[derive(Default)]
pub struct SerialTool {
    window: nwg::Window,
    input: nwg::TextInput,                           //输入框
    check: nwg::Button,                              //查找串口
    combobox_ports: nwg::ComboBox<&'static str>,     //端口下拉菜单
    combobox_baud_rate: nwg::ComboBox<&'static str>, //波特率下拉菜单
    open: nwg::Button,                               //开启串口
    close: nwg::Button,                              //关闭串口
    clear: nwg::Button,                              //清屏
    read: nwg::Button,                               //读取
    stop_read: nwg::Button,                          //停止读取
    write: nwg::Button,                              //写入
}

impl SerialTool {
    pub fn init(&self) {
        nwg::init().expect("msg");
        let _app = SerialTool::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events();
    }

    fn check(&self) {
        let ports = serial_port::get_ports();
        if ports.is_empty() {
            println!("No found serial ports!");
            nwg::error_message("错误", "No found serial ports!");
        } else {
            println!("{:?}", ports);
            self.combobox_ports.set_collection(ports);
        };
        self.close_port();
    }

    fn open(&self) {
        match (
            self.combobox_ports.selection_string(),
            self.combobox_baud_rate.selection_string(),
        ) {
            (Some(port_name), Some(baud_rate)) => {
                let baud_rate: u32 = baud_rate.parse().unwrap();
                self.open.set_enabled(false);

                PORT.lock().unwrap()[0] = serial_port::open(&port_name, baud_rate, 10);
            }
            _ => {
                nwg::error_message("Error", "请选择端口和波特率");
                self.open.set_enabled(true);
                PORT.lock().unwrap()[0] = None;
            }
        }
    }

    fn close_port(&self) {
        self.read.set_enabled(true);
        self.open.set_enabled(true);
        serial_port::stop_read();
        PORT.lock().unwrap()[0] = None;
    }
    fn clear(&self) {
        serial_port::clear();
    }
    fn read(&self) {
        let p = &PORT.lock().unwrap()[0];
        serial_port::read(p);
        self.read.set_enabled(false);
        self.stop_read.set_enabled(true);
    }
    fn stop_read(&self) {
        serial_port::stop_read();
        self.stop_read.set_enabled(false);
        self.read.set_enabled(true);
    }
    fn write(&self) {
        let p = &PORT.lock().unwrap()[0];

        let content = self.input.text();
        if content.is_empty() {
            nwg::error_message("Error", "请输入要写入的数据");
        } else {
            serial_port::write(p, content);
        }
    }
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

mod serial_tool {
    use super::*;
    use native_windows_gui as nwg;
    use std::{cell::RefCell, ops::Deref, rc::Rc};

    pub struct SerialToolUi {
        inner: Rc<SerialTool>,
        default_handler: RefCell<Vec<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<SerialToolUi> for SerialTool {
        fn build_ui(mut data: SerialTool) -> Result<SerialToolUi, nwg::NwgError> {
            nwg::Window::builder()
                .size((640, 480))
                .position((300, 300))
                .title("串口监视器")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .parent(&data.window)
                .size((480, 300))
                .position((10, 10))
                .build(&mut data.input)?;

            nwg::Button::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 10))
                .text("查找串口")
                .build(&mut data.check)?;

            nwg::ComboBox::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 60))
                .build(&mut data.combobox_ports)?;

            nwg::ComboBox::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 110))
                .collection(vec![
                    "300", "600", "750", "1200", "2400", "4800", "9600", "19200", "38400", "57600",
                    "115200", "230400", "460800", "500000", "921600", "1000000", "2000000",
                ])
                .build(&mut data.combobox_baud_rate)?;
            nwg::Button::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 160))
                .text("连接串口")
                .build(&mut data.open)?;

            nwg::Button::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 210))
                .text("关闭串口")
                .build(&mut data.close)?;

            nwg::Button::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 260))
                .text("清屏")
                .build(&mut data.clear)?;

            nwg::Button::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 310))
                .text("读取串口")
                .build(&mut data.read)?;

            nwg::Button::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 360))
                .text("停止读取")
                .enabled(false)
                .build(&mut data.stop_read)?;

            nwg::Button::builder()
                .parent(&data.window)
                .size((100, 30))
                .position((500, 410))
                .text("写入")
                .build(&mut data.write)?;

            let ui = SerialToolUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            use nwg::Event;

            // Events
            let handles = [&ui.window.handle];
            for handle in handles.iter() {
                let evt_ui = Rc::downgrade(&ui.inner);
                let handle_events = move |evt, _evt_data, handle| {
                    if let Some(ui) = evt_ui.upgrade() {
                        match evt {
                            Event::OnButtonClick => {
                                if &handle == &ui.check {
                                    SerialTool::check(&ui);
                                } else if &handle == &ui.open {
                                    SerialTool::open(&ui);
                                } else if &handle == &ui.close {
                                    SerialTool::close_port(&ui);
                                } else if &handle == &ui.clear {
                                    SerialTool::clear(&ui);
                                } else if &handle == &ui.read {
                                    SerialTool::read(&ui);
                                } else if &handle == &ui.stop_read {
                                    SerialTool::stop_read(&ui);
                                } else if &handle == &ui.write {
                                    SerialTool::write(&ui);
                                }
                            }
                            Event::OnComboxBoxSelection => if &handle == &ui.combobox_ports {},
                            Event::OnWindowClose => {
                                if &handle == &ui.window {
                                    SerialTool::exit(&ui)
                                }
                            }
                            _ => {}
                        }
                    }
                };
                ui.default_handler
                    .borrow_mut()
                    .push(nwg::full_bind_event_handler(handle, handle_events));
            }

            Ok(ui)
        }
    }
    impl Drop for SerialToolUi {
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
        }
    }
    impl Deref for SerialToolUi {
        type Target = SerialTool;

        fn deref(&self) -> &SerialTool {
            &self.inner
        }
    }
}
