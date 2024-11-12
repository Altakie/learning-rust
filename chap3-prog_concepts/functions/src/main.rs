fn main() {
    another_func(5, 'm');
    statements_and_expressions();
    let i = 3;
    let result = implicit_returns(3);
    println!("The value of i is {i} and the fn returns {result}");
}

fn another_func(x: i32, unit_label: char) {
    println!("The value of x is {x}{unit_label}");
}

fn statements_and_expressions() {
    let x = 0; // Statement
    let y = {
        let x = 6;
        x + 1
    };

    println!("The value of x is {x}, and the value of y is {y}");
}

fn implicit_returns(i : i32) -> i32 {
    i * i
}
