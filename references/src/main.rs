fn main() {
    // The rules of references:
    // - At any given time, you can have either one mutable reference or any number of immutable references.
    // - References must always be valid.


    // pass as reference, instead of ownership:
    {
        let mut s1 = String::from("hello");
        let len = calculate_length(&mut s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    // data race example:
    {
        let mut data = String::from("hello");
        {
            let r1 = &mut data;
            println!("{}", r1);
        }
        // this works because scope of first reference has ended
        let r2 = &mut data;
        println!("{}", r2);
    }

    // immutable vs mutable reference scope example
    {
        let mut s = String::from("hello");

        {
            let r1 = &s;
            let r2 = &s;
            println!("{}, {}", r1, r2);
        }
        // this works, because scope of immutable references has ended
        let r3 = &mut s;
        println!("{}", r3);
    }

    // dangling reference
    {
        let reference_to_nothing = dangle();
    }
}

// parameter as reference
fn calculate_length(s: &mut String) -> usize {
    s.push_str("string");
    s.len()
}

// to fix: return String instead of dangling reference
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}