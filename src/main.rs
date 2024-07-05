use std::io;

// These constatns hold the information about the factors to check, as well
// as their corresponding strings to print if a factor has been found in one of the numbers.
const MAX_FACTORS: usize = 3;

struct FactorString<'a> {
    factor: u32,
    string: &'a str,
}

// I was hoping to simplify by making an array of tuples of the form (3, "Fizz"), but sadly that
// doesn't seem possible. As such, having an array of structs is the best I could manage.
const FACTORS_AND_STRINGS: [FactorString; MAX_FACTORS] =
                                                        [ FactorString { factor: 3, string: "Fizz" },
                                                          FactorString { factor: 5, string: "Buzz" },
                                                          FactorString { factor: 7, string: "Pop" } ];

fn main() {
    println!("FizzBuzz");

    println!("Please enter the number of trials to perform.");

    let mut max_nums = String::new();
    
    // Read the user input for the maximum number of Fizz Buzz trials to perform.
    io::stdin()
        .read_line(&mut max_nums)
        .expect("Failed to read line");

    // Convert the max_nums String to an integer for the program to use.
    let max_nums: u32 = match max_nums.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid integer provided.");
            return;
        }
    };

    // Check the factors of each number in the range of [1, max_nums]
    for count in 1..max_nums+1 {
        // This will hold the Fizz and Buzz strings if a number that holds one of our factors is
        // found.
        let mut result_string = String::new();

        // Check each factor in each number.
        for factor_data in FACTORS_AND_STRINGS {
            // If a factor has been found (i.e., the remainder is 0 when the number is divided by
            // it), we need to push the corrresponding string into the result_string to print at
            // the end.
            if count % factor_data.factor == 0 {
                result_string.push_str(factor_data.string);
            }
        }

        // Once all of the FACTORS have been checked, we either print the number if
        // it is NOT divisible by any of the FACTORS we've checked, or we print the
        // factor string.

        // If there are no other characters in the result_string, we can just print the number.
        if result_string.len() == 0 {
            println!("{}", count);
        } else {
            println!("{}", result_string);
        }
    }
}
