use std::io;

fn main() {
    loop {
        let mut temp = String::new();
        println!("Input Fahrenheit Temperature!");
        io::stdin().read_line(&mut temp).expect("Failed to input");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let conv_temp: f32 = convert_f2_c(temp);
        let nth_sum: i128 = nth_fibonacci(temp as u32);
        println!("{conv_temp}");
        println!("{nth_sum}");
    }
}

fn convert_f2_c(temp: f32) -> f32 {
    (temp - 32.0) * (9.0 / 5.0)
}

fn nth_fibonacci(mut num: u32) -> i128 {
    let mut pre_num: i128 = 1;
    let mut sec_num: i128 = 1;
    let mut next_num: i128 = 0;
    if num <=2 {return pre_num;}
    while num > 2 {
        next_num = sec_num + pre_num;
        pre_num = sec_num;
        sec_num = next_num;
        num = num - 1;
    };
    next_num 
}