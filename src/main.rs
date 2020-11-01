use chrono::{Local, Timelike};
use hidapi::HidApi;
use std::time::Duration;
use std::thread;

fn main() {
    let api = HidApi::new().expect("Failed to unwarp HID API");
    let device_info = api.device_list()
        .find(|&d| d.vendor_id() == 0x04D8 && d.product_id() == 0xEB2D &&
            d.usage() == 0x0061 && d.usage_page() == 0xFF60).expect("Failed to find device");
    let device = device_info.open_device(&api).expect("Failed to open device");

    // Write time info
    let dt = Local::now();
    device.write(&[0x00, 0x01, 0x01, dt.hour() as u8, dt.minute() as u8]).expect("Failed to write to device");
    thread::sleep(Duration::from_millis(100));

    // Write notification info
    device.write(&[0x00, 0x02, 0x01, 0]).expect("Failed to write to device");
}
