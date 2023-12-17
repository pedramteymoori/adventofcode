use std::io;

fn extract_chars(input_line : &str) -> (i32, i32){
    let digits :[&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut first: i32 = -1;
    let mut last: i32 = -1;

    let chars : Vec<char>= input_line.chars().collect();

    for (char_index, c) in chars.iter().enumerate() {
        if c.is_digit(10) {
            let digit_value = match c.to_digit(10){
                Some(d) => d as i32,
                None => {
                    println!("cannot cast into a number");
                    continue
                }
            };

            last = digit_value;
            if first == -1 {
                first = digit_value;
            }
        }


        // This is the worst code I've written in past 4 years. Consider I'm a newbie in Rust when reading.
        for (digit_int,digit) in digits.iter().enumerate() {
            if (char_index + digit.len() -1) > (chars.len() -1 ){
                continue
            }

            let mut found :bool= true;
            for digit_index in 0..digit.len(){
                if !found{
                    break
                }

                for (i, char_index_2) in chars[char_index..].to_vec().iter().enumerate() {
                    if i>= digit.len(){
                        break
                    }

                    if chars.get(i).unwrap() != &digit.chars().nth(digit_index+i).unwrap(){
                        found = false;
                        break;
                    }
                }

                if found == true{
                    last = digit_int as i32;
                    if first == -1 {
                        first = digit_int as i32;
                    }
                    break
                }
            }
        }
    }

    return (first, last)
}

fn main(){
    let mut sum = 0;

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("failed to read line");

        let input_line = input_line.trim();

        if input_line == ";" {
            break
        }

        let (first, last) = extract_chars(input_line);
        if first != -1 && last != -1 {
            sum += (first * 10) + last;
        }

    }
    println!("{}", sum);
}