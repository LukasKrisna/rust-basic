pub fn say_hello() {
    println!("Hello first module!");
}

pub fn say_hello_second() {
    println!("say hello second on first module!");
}

pub mod second {
    pub mod third {
        pub fn say_hello() {
            super::super::say_hello_second()
        }
    }
}