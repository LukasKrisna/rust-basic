use crate::third::say_hello as say_hello_third;

pub fn say_hello() {
    println!("Hello second module!");
    
    say_hello_third();
}