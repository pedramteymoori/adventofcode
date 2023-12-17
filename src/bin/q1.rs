
use std::io;
fn main() {
    let mut sum = 0;

    loop{
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("failed to read line");

        let input_line = input_line.trim();

        if input_line == ";" {
            break
        }
        // shadowed

        let mut first: i32 = -1;
        let mut last: i32 = -1;

        for c in input_line.chars() {
            if !c.is_digit(10) {
                continue
            }

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
        if first != -1 && last != -1 {
            sum += (first * 10) + last;
        }

    }
    println!("{}", sum);
}
