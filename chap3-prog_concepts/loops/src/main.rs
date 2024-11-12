fn main() {
    loop_demo();
    while_loop();
}

fn loop_demo() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is {result}");
}

fn while_loop() {
    let mut counter = 0;

    loop {
        let condition = {counter <= 2};
        if condition == false {
            break;
        }

        println!("Counter is {counter}");

        counter += 1;
    }
}
