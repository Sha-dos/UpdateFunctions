use std::thread;
use std::time::Duration;

pub fn update(duration: Duration, function: fn()) {
    function();
    thread::sleep(duration);
    update(duration, function);
}
