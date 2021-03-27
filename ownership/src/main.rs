fn main() {
    let s = String::from("hello");
    let y = s.clone();

    takes_ownership(s);

    // will fail, because s is in heap, so it's moved to takes_ownership:
    //println!("{}", s);

    // cloned s works here
    println!("{}", y);

    let x = 5;
    makes_copy(x);

    // will not fail, because value in stack
    println!("{}", x);

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello s2");

    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);

    // will fail, because s2 is in heap, so it's moved to takes_and_gives_back
    //println!("{}", s2);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    let s = String::from("hello s1");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}
