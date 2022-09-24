mod raw_identifier;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    string_clone();
    // guess_number();
    string_1();
    other_slice();
    // println!("{}", raw_identifier::raw_id())
    let user = build_user(String::from("johann"), String::from("iinux"));
    println!("{:?}", user);
    println!("{:#?}", user);
    dbg!(user);
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
    for element in months.iter() {
        println!("{}", element);
    }
    for number in (1..4).rev() {
        println!("{}", number);
    }


    println!("{}", another_function(5));

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    assert_eq!(result, 20);


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
