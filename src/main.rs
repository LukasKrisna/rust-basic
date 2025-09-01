fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn test_variable() {
    let name = String::from("Lukas Krisna");
    let age:i32 = 21;
    println!("Hello {}, usia anda {}", name , age )
}

#[test]
fn test_mutable_variable() {
    let mut name = String::from("Lukas Krisna");

    println!("Hello {}", name);

    name = String::from("Lukas");
    let age:i32 = 21;
    println!("Hello {}, usia anda {}", name , age )
}

#[test]
fn shadowing() {
    let x = "lukas";
    println!("Hello {}", x);

    let x = 10;
    println!("Hello {}", x);
}