use std::io;

const WOW : u32 = 101;

fn main() {
    println!("The value of WOW (constant) is : {WOW}");

    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    let x = x + 1; // Variable Shadowing

    {
    let x = x * 2; // Variable is only shadowed until the scope ends
    println!("The value of x within the inner scope is : {x}");
    }

    // Scope ended, so variable now back to normal value
    println!("The value of x within the outer scope is : {x}");

    // Char's aren't 1 byte, but represent unicode and are thus 4 bytes
    let c : char = 'c';
    let happy_face = '😊'; 
    println!("Two chars combined {c} and {happy_face}");

    // tuples can be multiple types, but are of fixed size when created
    let tup : (char, u32, i32, f64) = ('a', 10, -11, 1.001);
    let (_x, _y, _z, w) = tup;
    let tup2 = tup.1;
    let tup4 = tup.3;
    println!("The 4th value of the tuple is : {w}, or {tup4}. The second value is {tup2}");

    // Arrays of fixed size, syntax like every other language
    let arr : [i32; 5] = [1, 2, 3, 4, 5];
    let _third = arr[2];
    
    let _default_arr = [3; 5]; // Array has default value of 3 with size 5
    
    println!("Please enter an array number!");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index : usize = index.trim().parse().expect("Index is not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is {element}");

}
