use lazy_static::lazy_static;
use serialport::{self, SerialPort};
use std::{str, thread, time::Duration};

extern crate native_windows_gui as nwg;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<bool>> = Mutex::new(vec![false]);
}

pub fn get_ports() -> Vec<&'static str> {
    let mut ports: Vec<&'static str> = vec![];

    let ports_info = serialport::available_ports().expect("无法获取串口信息");
    for port in ports_info {
        ports.push(Box::leak(port.port_name.into_boxed_str()));
    }
    ports
}

pub fn open(port_name: &str, baud_rate: u32, time: u64) -> Option<Box<dyn SerialPort>> {
    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(time))
        .open();

    match port {
        Ok(port) => Some(port),
        Err(_) => None,
    }
}
pub fn close() {
    ARRAY.lock().unwrap()[0] = false;
}
pub fn read(port: &Option<Box<dyn SerialPort>>) {
    match port {
        Some(port) => {
            let mut port = port.try_clone().expect("msg");

            let mut serial_buf = vec![0, 100];

            ARRAY.lock().unwrap()[0] = true;

            thread::spawn(move || loop {
                if ARRAY.lock().unwrap()[0] {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(_) => {
                            let a = str::from_utf8(&serial_buf).expect("msg").to_owned();
                            print!("{a}");
                        }
                        Err(_) => {
                            //eprintln!("{:?}", e);
                        }
                    }
                } else {
                    break;
                }
            });
        }
        None => {
            nwg::error_message("Error", "无法读取串口");
        }
    };
}

pub fn write(port: &Option<Box<dyn SerialPort>>, content: String) {
    match port {
        Some(port) => {
            let mut port = port.try_clone().expect("msg");

            let content = Box::leak(content.into_boxed_str());

            thread::spawn(move || match port.write(content.as_bytes()) {
                Ok(_) => {
                    println!("已写入{}", content);
                }
                Err(_) => {
                    println!("写入{}失败", content);
                }
            });
        }
        None => {
            nwg::error_message("Error", "无法读取串口");
        }
    };
}

pub fn clear() {
    print!("\x1b[2J");
    print!("\x1b[H");
    println!("");
    print!("\x1b[H");
}
