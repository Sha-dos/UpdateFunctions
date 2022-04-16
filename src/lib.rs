use std::thread;
use std::time::Duration;

pub fn update(duration: Duration, function: fn()) {
    function();
    thread::sleep(duration);
    update(duration, function);
}

pub fn update_true(duration: Duration, statement: bool, function: fn()) {
    if statement == true { 
        function(); 
        thread::sleep(duration);
        update_true(duration, statement, function)
    }
    else{ return; }
}
