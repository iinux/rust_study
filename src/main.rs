use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
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

    println!("The secret number is: {}", secret_number);

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
