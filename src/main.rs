use std::io;
use std::io::Write;

fn main() {
    fahrenheit_to_celsius();
    celsius_to_fahrenheit();
    fib_test();
    twelve_days_of_christmas();
}

fn fahrenheit_to_celsius() {
    loop {
       let mut inp = String::new();

       print!("Celsius-> ");
       io::stdout().flush().expect("Flush failed");
       io::stdin()
           .read_line(&mut inp)
           .expect("Failed to readline");
        let mut inp: f32 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("You must enter a valid number");
                continue;
            }
        };

        inp = f_to_c(inp);
        println!("{inp} F°");
        break;
    }
}

fn celsius_to_fahrenheit() {
    loop {
        let mut inp = String::new();

        print!("Fahrenheit-> ");
        io::stdout().flush().expect("Flush failed");
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to readline");
        let mut inp: f32 = match inp.trim().parse() {
            Ok(num)  => num,
            Err(_) => {
                eprintln!("You must enter a valid number");
                continue;
            }
        };

        inp = c_to_f(inp);
        println!("{inp} C°");
        break;
    }
}

/**
 * Practice One
 * Convert temperature between Fahrenheit and Celsius
 */
fn f_to_c(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

fn c_to_f(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn fib_test() {
    for n in 0..10 {
        println!("Fib {n} -> {}", fib(n));
    }
}

/**
 * Fibonacci sequence
 * 1 1 2 3 5 8 13
 */
fn fib(n:i32) -> i32 {
    if n == 1 {
        return 1
    } else if n <= 0 {
        return 0
    }
    fib(n-2) + fib(n-1)
}

fn twelve_days_of_christmas() {
    let lyrics = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let day_str = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "either", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for day in 0..=11 {
        println!("On the {} day of Christmas,\nmy true love sent to me", day_str[day]);
        for l in (0..=day).rev() {
            if l != 0 || day == 0 {
                println!("{},", lyrics[l]);
            } else {
                println!("and {}", lyrics[l]);
            }
        }
        println!("");
    }
}
