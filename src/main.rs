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

#[test]
fn while_loop() {
    let mut number = 0;
    while number <= 10 {
        if number % 2 == 0 && number != 0  {
            println!("{} is divisible by 2", number);
        }
        number += 1;
    }
}

#[test]
fn array_iteration_while() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("{}", array[index]);
        index += 1;
    }
}

#[test]
fn for_loop_array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in array.iter() {
        println!("{}", i);
    }
}

#[test]
fn range() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("Start range: {}", range.start);
    println!("End range: {}", range.end);

    for i in range {
        println!("{}", array[i]);
    }

    println!("  ");
    for i in 0..5 { // range exclusive
        println!("{}", i);
    }
    println!("  ");
    let range_inclusive = 0..=5;
    println!("Start range: {}", range_inclusive.start());
    println!("End range: {}", range_inclusive.end());
    for i in range_inclusive { // range inclusive
        println!("{}", i);
    }
}

fn say_hello() {
    println!("Hello");
}

#[test]
fn test_say_hello() {
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye, {} {}", first_name, last_name);
}

#[test ]
fn test_say_goodbye() {
    say_goodbye("Lukas", "Krisna");
    say_goodbye("Lorem", "Krisna");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial() {
    let result = factorial_loop(5  );

    println!("{}", result) ;
}

fn print_text_recursive(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text_recursive(value, times - 1);
}

#[test]
fn test_print_text_recursive() {
    print_text_recursive(String::from("Lukas"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result) ;
}

// function ownership
fn print_number(number: i32) {
    println!("Number: {}", number);
}

fn hi(name: String){
    println!("Hi, {}", name);
}
#[test]
fn test_hi_print_number() {
    let number = 10;
    print_number(number);
    println!("{}", number); // number dicopy pada stack

    let name = String::from("Lukas");
    hi(name);
    // println!("{}", name); // error karena ownership name pindah pada parameter function hi
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Lukas");
    let last_name = String::from("Krisna");

    let full_name = full_name(first_name, last_name);
    println!("{}", full_name);
    // println!("{}", first_name); error, return value first_name berpindah
}

// mengembalikan ownership
fn full_name_2(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_full_name2() {
    let first_name = String::from("Lukas");
    let last_name = String::from("Krisna");

    let (a, b, name) = full_name_2(first_name, last_name);
    println!("{}", name);
    println!("{}", a);
    println!("{}", b);
}

// reference
fn full_name_reference(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name_reference() {
    let first_name = String::from("Lukas");
    let last_name = String::from("Krisna");

    let full_name = full_name_reference(&first_name, &last_name);
    println!("{}", full_name);
    println!("{}", first_name);
}

// borrowing error
/* fn change_value(value: &String) -> String {
     value.push_str("Lukas"); //error cannot borrow immutable local variable
} */

#[test]
fn test_change_value_error() {
    let mut value = String::from("Lukas");
    // change_value(&value);
    println!("{}", value);
}

// borrowing change value
fn change_value(value: &mut String) {
     value.push_str("Lukas"); //error cannot borrow immutable local variable
}

#[test]
fn test_change_value() {
    let mut value = String::from("Krisna");
    change_value(&mut value);
    change_value(&mut value);
    change_value(&mut value);
    println!("{}", value);
}

// dangling pointer
/* fn get_full_name(first_name: &String, last_name: &String) -> &String {
    let name = format!("{} {}", first_name, last_name);
    return &name;
} */

// #[test]
/* fn test_get_full_name() {
    let first_name = String::from("Lukas");
    let last_name = String::from("Krisna");

    let full_name = get_full_name(&first_name, &last_name);
    println!("{}", full_name);
} */

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Lukas");
    let last_name = String::from("Krisna");

    let full_name = get_full_name(&first_name, &last_name);
    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &array[..];
    println!("{:?}", slice);

    let slice2 = &array[0..5];
    println!("{:?}", slice2);

    let slice3 = &array[5..];
    println!("{:?}", slice3);
}

#[test]
fn string_slice() {
    let name: String = String::from("Lukas Krisna");
    let first_name = &name[0..5];
    println!("{:?}", first_name);

    let last_name = &name[6..];
    println!("{:?}", last_name);
}

struct Person{
    first_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn test_sturct_person() {
    // let first_name = String::from("Lukas"); <- init shorthand

    let person: Person = Person {
        // first_name, ownership berpindah
        first_name: String::from("Lukas"),
        last_name: String::from("Krisna"),
        age: 21,
    };

    print_person(&person);

    // let person2: Person = Person{..person} hati hati dengan perpindahan ownership data di heap
    let person2: Person = Person{ // struct update syntax
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };

    print_person(&person2);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point: GeoPoint = GeoPoint(123.3, 456.6);
    println!("long: {}", geo_point.0);
    println!("lat: {}", geo_point.1);
}

// struct tanpa field
struct Nothing;

#[test]
fn test_nothing() {
    let _nothing: Nothing = Nothing;
    let _nothing1: Nothing = Nothing {};
}

// method
impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello, {}!, my name is {}",name, self.first_name);
    }
}

#[test]
fn test_method() {
    let person: Person = Person {
        // first_name, ownership berpindah
        first_name: String::from("Lukas"),
        last_name: String::from("Krisna"),
        age: 21,
    };

    person.say_hello("Lorem");
}

// Associated Functions yang bukan method
impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_new_geo_point() {
    let geo_point: GeoPoint = GeoPoint::new(-456.6, -123.3);
    println!("long: {}", geo_point.0);
    println!("lat: {}", geo_point.1);
}