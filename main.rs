use std::io::stdin;

fn select_xor_value(result_byte: u8, multi_byte: u8) -> u8 {
    for value in 0..=255 {
        if multi_byte ^ value == result_byte {
            return value
        }
    }
    0
}

fn main() {
    let mut result = String::new();
    match stdin().read_line(&mut result) {
        Ok(n) => result = result.trim_end().to_string(),
        Err(error) => println!("Error: {}", error),
    } println!();
    let result_bytes = u32::from_str_radix(&result, 16).unwrap().to_be_bytes();

    let mut input_data = String::new();
    match stdin().read_line(&mut input_data) {
        Ok(n) => input_data = input_data.trim_end().to_string(),
        Err(error) => println!("Error: {}", error),
    }

    let mut main_values: [u8; 4] = [0; 4];
    for i in 0..4 {
        main_values[i] = input_data.as_bytes()[i]
    }
    let seconds = &input_data[4..];
    let mut count = 0;
    for byte in seconds.as_bytes() {
        main_values[count] ^= byte;
        count = (count + 1) % 4;
    }

    let mut bytes = Vec::new();
    let mut now_changing = seconds.len() % 4;
    let barrier = match now_changing {
        0 => 4,
        3 => 5,
        2 => 6,
        1 => 7,
        _ => 4
    };
    for _ in 0..barrier {
        bytes.push(select_xor_value(
            result_bytes[now_changing],
            main_values[now_changing]));
        main_values[now_changing] = result_bytes[now_changing];
        now_changing = (now_changing + 1) % 4;
    }

    let mut result_string = String::new();
    for byte in bytes {
        result_string += &format!("{:0>2x}", byte).to_string();
    }
    println!("{}", result_string)
}