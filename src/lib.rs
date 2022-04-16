use std::thread;
use std::time::Duration;

/// Update is a function that calls or "updates" another function
pub fn update(duration: Duration, function: fn()) {
    function();
    thread::sleep(duration);
    update(duration, function);
}

/// Update_True is a function that calls or "updates" another function if a bool is true
pub fn update_true(duration: Duration, statement: bool, function: fn()) {
    if statement == true { 
        function(); 
        thread::sleep(duration);
        update_true(duration, statement, function)
    }
    else{ return; }
}
