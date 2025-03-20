use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use std::thread;
use std::time::Duration;

fn main() {
    let event_handler = DeviceEventsHandler::new(Duration::from_millis(10)).expect("Couldn't init event loop");

    let _kb_listener = event_handler.on_key_up(|keycode: &Keycode| {
        println!("Key pressed: {:?}", keycode);
    });

    loop {
        thread::sleep(Duration::from_secs(1000));
    }
}
