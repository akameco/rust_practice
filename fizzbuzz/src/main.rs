macro_rules! fizz_buzz {
    ($range:expr) => {
        for i in $range {
            if i % 3 == 0 && i % 5 == 0 {
                println!("FizzBuzz");
            } else if i % 3 == 0 {
                println!("Fizz");
            } else if i % 5 == 0 {
                println!("Buzz");
            } else {
                println!("{}", i);
            }
        }
    };
}

fn main() {
    fizz_buzz!(1..=100);
}