use std::process::Command;
use std::env;

fn main() {
    set_keyboard_colour();
    
}

fn set_keyboard_colour(){

    let mut keyboard = Command::new("sudo");
    keyboard.arg("./g810-led");
    keyboard.arg("-dv");
    keyboard.arg("046d");
    keyboard.arg("-dp");
    keyboard.arg("c33c");
    keyboard.arg("-tuk");
    keyboard.arg("1");
    keyboard.arg("-a");
    keyboard.arg("FFFFFF");
    keyboard.status().expect("process failed to execute");
}

