fn main() {
  basic_ifs();
  if_in_let();
}

fn basic_ifs() {

    let number = 3;

    if number < 5 {
      println!("The condition was true"); 
    }
    else {
      println!("The condition was false"); 
    }
}

fn if_in_let() {
  let condition: bool = true;
  let number = if condition {5} else {2};

  println!("The number is {number}");
}
