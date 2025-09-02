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

#[test]
fn number() {
    let a: i32 = 5;
    let b: f64 = 6.1;

    println!("Number a(i32): {} and b(f64): {}", a, b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    let a: i32 = a as i32;
    println!("{}", a);


    // integer overflow
    let b: i64 = 1000000000;
    let b: i8 = b as i8;
    println!("{}", b);
}

#[test]
fn numeric_operator() {
    let a = 20;
    let b = 10;
    println!("{}", a * b);
    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a / b);
    println!("{}", a % b);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    a *= 2;
    println!("{}", a);

    a += 5;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;
    println!("{}, {}", a, b);
}

#[test]
fn comparison() {
    let a: bool = 10 < 20;
    println!("is 10 less than 20? {}", a);
}

#[test]
fn boolean_operator() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = a && b;
    let d: bool = a || b;
    let e: bool = !a;
    println!("{}, {}, {}", c, d, e);
}

#[test]
fn char_type() {
    let a = 'z';
    let b: char = 'y';
    println!("{}, {}", a, b);
}

#[test]
fn tuple() {
    let data: (i32, f64, bool) = (1, 1.2, false);
    let mut data2: (i32, f64) = (1, 1.2);
    println!("{:?}", data);
    println!("{}, {}", data.1, data.2);

    // destructuring tuple
    let (a, _, c) = data;
    println!("{}, {}", a, c);

    data2.0 = 10;
    data2.1 = 2.1;
    println!("{}, {}", data2.0, data2.1);

}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    // empty tuple
    let result = unit();
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test)
}