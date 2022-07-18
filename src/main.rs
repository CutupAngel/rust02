use std::io;

fn main() {
    loop {
        let mut sum = 0;
        let mut temp = String::new();
        println!("Input Fahrenheit Temperature!");
        io::stdin().read_line(&mut temp).expect("Failed to input");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let conv_temp: f32 = convert_f2_c(temp);
        println!("{conv_temp}");
        for num in 1..(temp+1.0) as u32 {
            sum += num;
        }
        println!("{sum}");
    }
}

fn convert_f2_c(temp: f32) -> f32 {
    (temp - 32.0) * (9.0 / 5.0)
}
