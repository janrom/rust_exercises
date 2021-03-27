fn main() {
    let x = test(2);
    println!("{}", x);
}

fn test(x: i32) -> i32 {
    x + 1
}