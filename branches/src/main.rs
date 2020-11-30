fn some_bool_func() -> bool{
    true
}

fn main() {

    let x: bool = some_bool_func();
    let truth_number = if x { 1 } else { 0 };
    println!("The truth number is {}", truth_number);

}
