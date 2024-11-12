use std::io;

fn main() {
    let input_num = loop {
        println!("Please input the sequence number of the fibonacci number you want");
        let mut input_num = String::new();

        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read line");

        let input_num: u32 = match input_num.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number");
                continue;
            }
        };

        break input_num;
    };

    println!("Fibonacci is {}", fibonacci(input_num));
}

fn fibonacci(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    for _ in 1..n {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return first;
}
