
use std::io::{self, BufRead};
    pub fn run() {
        let stdin = io::stdin();

        let mut sum = 0;
        for line in stdin.lock().lines() {
            let str = line.unwrap();
            if str == ";" {
                break
            }

            let mut first: i32 = -1;
            let mut last: i32 = -1;

            for c in str.chars() {
                if !c.is_digit(10) {
                    continue
                }

                let digit_value = c.to_digit(10).unwrap() as i32;
                last = digit_value;
                if first == -1 {
                    first = digit_value;
                }
            }
            sum += (first * 10) + last;
        }
        println!("{}", sum);
    }
