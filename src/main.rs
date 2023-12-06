use std::fs::read_to_string;

fn main() {
    let mut first_digit: Option<u16> = None;
    let mut last_digit: Option<u16> = None;
    let mut first: bool = true;
    let mut sum: u32 = 0;

    for line in read_to_string("./src/input.txt").unwrap().lines() {
        // println!("{}",line);
        for c in line.chars() {
            // println!("{}",c);
            let is_number: Result<u16, _> = c.to_string().parse();
            match is_number {
                Ok(number) => {
                    // println!("{} is a number",number);
                    if first {
                        first_digit = Some(number);
                        first = false;
                    }
                    last_digit = Some(number);
                },
                Err(_err) => {} 
            }
        }
        first = true;
        if let (Some(first_digit),Some(last_digit)) = (first_digit,last_digit) {
            // println!("first digit {}, last digit {}",first_digit,last_digit);
            let sumword = format!("{}{}",first_digit,last_digit).parse().unwrap_or(0);
            sum = sum + sumword;
            // println!("result of the line {}", sumword);
        } else {
            // println!("not initialized");
        }
        first_digit = None;
        last_digit = None;
    }
    println!("total is {}",sum);
}
