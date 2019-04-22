// Problem statement: http://www.rosettacode.org/wiki/FizzBuzz

fn main() {
    println!("--FizzBuzz Program--");
    for n in 1..=100 {
        match n {
            x if x % 5 == 0 && x % 3 == 0 => println!("FizzBuzz"),
            x if x % 5 == 0  => println!("Buzz"),
            x if x % 3 == 0 => println!("Fizz"),
            x => println!("{}", x),
        }
    }
}

