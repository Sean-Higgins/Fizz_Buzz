use std::io;

const MAX_DIVISORS: usize = 3;

const DIVISORS: [u32; MAX_DIVISORS] = [3, 5, 7];
const DIVISOR_STRINGS: [&str; MAX_DIVISORS] = ["Fizz", "Buzz", "Pop"];

fn main() {
    let mut max_nums = String::new();

    println!("FizzBuzz");

    println!("Please enter the number of trials to perform.");

    io::stdin()
        .read_line(&mut max_nums)
        .expect("Failed to read line");

    let max_nums: u32 = match max_nums.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid integer provided.");
            return;
        }
    };

    for count in 1..max_nums+1 {
        let mut result_string = String::new();

        for divisor_count in 0..MAX_DIVISORS {

            if count % DIVISORS[divisor_count] == 0 {
                result_string.push_str(DIVISOR_STRINGS[divisor_count]);
            }
        }

        // Once all of the DIVISORS have been checked, we either print the number if
        // it is NOT divisible by any of the DIVISORS we've checked, or we print the
        // divisor string.
        if result_string.len() == 0 {
            println!("{}", count);
        } else {
            println!("{}", result_string);
        }
    }
}
