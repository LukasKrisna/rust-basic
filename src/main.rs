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

#[test]
fn array() {
    let mut a: [i32; 3] = [1, 2, 3];
    println!("{:?}", a);

    a[0] = 2;
    println!("{:?}", a);

    //length array
    let length: usize = a.len();
    println!("length a is {}", length);
}

#[test]
fn two_dimentional_array(){
    let matrix: [[i32; 2]; 3] = [
        [1, 2],
        [3, 4],
        [5, 6]
    ];
    println!("{:?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}

const MINIMUM:i32 = i32::MIN;
#[test]
fn constant() {
    const PI:f64 = 3.14;
    println!("PI is {}", PI);

    // outer scope
    println!("{}", MINIMUM);
}

#[test]
fn variable_scope() {
    let a = 1; // variable scope

    { // inner scope
        println!("inner a: {}", a);
        let b = 2;
        println!("inner b: {}", b);
    }

    //println!("inner b: {}", b); // error
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Lukas");
    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Krisna");
    println!("{}, {}", a, b);
}

#[test]
fn string() {
    let name: &str = " Lukas Krisna 😃 ";
    let trim: &str = name.trim();
    println!("{}", name);
    println!("{}", trim);

    let mut username: &str = "Lukas";
    println!("{}", username);
    username = "Krisna";
    println!("{}", username);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Lukas");
    name.push_str(" Krisna");
    println!("{}", name);

    // error karena .replace() membuat nilai baru pada heap
    // name.replace("Lukas", "Lorem")

    let lorem = name.replace("Lukas", "Lorem");
    println!("{}", lorem);
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses di sini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai dari sini

    { // b tidak bisa diakses di sini, belum dideklarasikan
        let b = 20; // b bisa diakses mulai dari sini
        println!("{}",b);
    } // scope b selesai, b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy() {
    let a = 10;
    let mut b = a;

    b = 20;
    println!("{}",b);
}

#[test]
fn ownnership_movement() {
    let nama: String = String::from("Lukas");

    // ownership dari nama dipindahkan ke nama_baru
    let nama_baru = nama;
    // nama tidak bisa diakses disini

    print!("{}", nama_baru);
}

#[test]
fn clone() { // clone adalah melakukan Copy untuk data pada heap
    let name1: String = String::from("Lukas");
    let name2: String = name1.clone();
    println!("{}", name1);
    println!("{}", name2);
}

#[test]
fn if_expression() {
    let nilai = 6;
    let hasil: &str;

    if nilai >= 8 {
        hasil = "lolos";
    } else if nilai >= 7 {
        hasil = "memenuhi syarat"
    } else {
        hasil = "belum lolos";
    }

    println!("{}", hasil);

    // if expression on let

    let result =  if nilai >= 8 {
        "lolos"
    } else if nilai >= 7 {
        "memenuhi syarat"
    } else {
        "belum lolos"
    };
}

#[test]
fn loop_expression() {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter > 10 {
            break;
        }else if counter % 2 == 0 {
            continue;
        }

        println!("{}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 0;

    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}