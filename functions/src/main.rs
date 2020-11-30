fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 6;
    let y = plus_one(x);
    println!("The plus_one() function gave us {} from {}.", y, x);
}
