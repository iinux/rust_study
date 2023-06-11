mod raw_identifier;
mod my_http;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use crate::raw_identifier::raw_id;
use std::collections::{HashMap, HashSet};

fn main() {
    string_clone();
    // guess_number();
    string_1();
    other_slice();
    tup_array();
    // println!("{}", raw_identifier::raw_id())
    let user = build_user(String::from("johann"), String::from("iinux"));
    println!("{:?}", user);
    println!("{:#?}", user);
    println!("age: {}", user.get_age());
    User::static_method();
    user.show();
    dbg!(user);
    // raw_id();
    my_http::send();

    let price = 99;
    double_price0(price);
    println!("{}", price);

    let price = 99;
    double_price(price);
    println!("{}", price);

    let mut price = 99;
    double_price2(&mut price);
    println!("{}", price);

    let price = 99;
    double_price3(&price);
    println!("{}", price);

    println!("{:?}", MyEnum::Day1);

    let result = get_discount(123);
    println!("{:?}", result);
    match result {
        Some(false) => println!("ok false"),
        Some(true) => println!("ok true"),
        None => println!("none"),
        _ => println!("default")
    }

    let me2 = MyEnum2::Name(String::from("i am string"));
    match me2 {
        MyEnum2::Name(val) => {
            println!("{} {:?}", val, val)
        }
    }

    let t = Data{value:100};
    println!("{}", t.value);
}

struct Data<T> {
    value:T
}

enum MyEnum2 {
    Name(String)
}

fn get_discount(price: i32) -> Option<bool> {
    if price > 100 {
        Some(true)
    } else {
        None
    }
}

#[derive(Debug)]
enum MyEnum {
    Day1,
    Day2,
    Day3
}

fn double_price0(price:i32){
    println!("{}", price);
}

fn double_price(mut price:i32) {
    price = price * 2;
    println!("{}", price);
}

fn double_price2(price: &mut i32) {
    *price = *price * 2;
    println!("{}", price);
}

fn double_price3(price:& i32) {
    println!("{}", price)
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        age: 0
    }
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: usize
}

trait Show {
    fn show(&self);
}

impl Show for User {
    fn show(&self) {
        println!("i am in trait")
    }
}

impl User {
    fn get_age(&self) -> usize {
        self.age
    }

    fn static_method() {
        println!("static method")
    }
}

fn other_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_1() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经是字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);

    let str1 = "zhang".to_string();
    let str2 = "san".to_string();
    let str3 = str1 + &str2;
    println!("{}", str3);
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn string_move() {
    let s1 = String::from("hello");
    // if uncomment will cause error
    // let s2 = s1;
    println!("{} world", s1);
}

fn string_test() {
    let mut s = "hello";
    let s2 = s;
    s = "1hello";
    let mut ss = String::from("hello");
    let ss2 = ss;
    ss = "1hello".to_string();
    ss.push_str(", world");
    println!("{} + {} + {} + {}", s, s2, ss, ss2);


}

fn tup_array() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{} {} {}", five_hundred, six_point_four, one);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {}", months[0], a[1]);

    let a = ["";3];
    println!("{:?}", a);

    for element in months.iter() {
        println!("{}", element);
    }
    for number in (1..4).rev() {
        println!("{}", number);
    }

    for number in 1..=5 {
        println!("{}", number);
    }

    let alphabet = vec!["A", "B", "C"];
    for m in alphabet.iter() {
        match m {
            &"A" => println!("GOOD"),
            _ => println!("{}", m)
        }
    }
    println!("{:?}", alphabet);

    for m in alphabet.into_iter() {
        match m {
            "A" => println!("GOOD"),
            _ => println!("{}", m)
        }
    }
    // println!("{:?}", alphabet);

    let mut alphabet = vec!["A", "B", "C"];
    for m in alphabet.iter_mut() {
        *m = match m {
            &mut "A" => "GOOD",
            _ => *m
        }
    }

    println!("vec {:?}", alphabet);

    println!("{}", another_function(5));

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    assert_eq!(result, 20);

    let mut v = Vec::new();
    v.push("a");
    v.push("b");
    v.push("c");
    println!("vec {:?}", v);

    if v.contains(&"b") {
        println!("vec contains b");
    }

    let x = v.remove(0);
    println!("{} {:?} {}", x, v, v[0]);

    let mut hm = HashMap::new();
    hm.insert("a", 1);
    hm.insert("b", 2);
    hm.insert("c", 3);
    hm.insert("d", "4".parse().unwrap());
    println!("hm={:?}", hm);

    match hm.get("a") {
        Some(v) => {
            println!("a={}", v)
        },
        None => {
            println!("none")
        }
    }

    for (k, v) in hm.iter() {
        println!("hm k={},v={}", k, v);
    }

    let a = hm.remove("a");
    println!("a={:?} hm={:?}", a, hm);

    let mut hs = HashSet::new();
    hs.insert("a");
    hs.insert("a");
    hs.insert("b");
    hs.insert("c");
    println!("hs={:?}", hs);

    for m in hs.iter() {
        println!("hs iter {}", m);
    }
    match hs.get("a") {
        None => println!("not found"),
        Some(data) => println!("{}", data)
    }
}

fn another_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);
    let y = {
        let x = 3;
        x + 1
    };
    println!("{} {}", x, y);

    5
}

fn guess_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        /*
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
            */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Err(_) => continue,
            Err(e) => {
                println!("ERROR: {}", e);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
